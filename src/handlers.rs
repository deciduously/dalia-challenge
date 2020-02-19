// handlers.rs
// Web route handlers and router

use super::*;
use askama::Template;
use chrono::prelude::*;
use flate2::{write::ZlibEncoder, Compression};
use hyper::{header, Body, Request, Response, StatusCode};
use std::{collections::HashMap, convert::TryInto, fs::File, io::prelude::*, path::PathBuf};
use url::form_urlencoded;

// Universal handler return type
pub type HandlerResult = AppResult<Response<Body>>;

// General handlers

/// Top-level handler that DEFLATE compresses and responds from a &[u8] body
/// If None passed to status, 200 OK will be returned
pub async fn bytes_handler(
    body: &[u8],
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(body)?;
    let compressed = e.finish()?;
    // Return response
    Ok(Response::builder()
        .status(status.unwrap_or_default())
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_ENCODING, "deflate")
        .body(Body::from(compressed))?)
}

/// Pass string to bytes_handler
pub async fn string_handler(
    body: &str,
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    bytes_handler(body.as_bytes(), content_type, status).await
}

/// Pass HTML string to string_handler
pub async fn html_str_handler(body: &str) -> HandlerResult {
    string_handler(body, "text/html", None).await
}

// Route handlers

/// Serve any image assets requested
pub async fn image(path_str: &str) -> HandlerResult {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    if let Some(ext) = path_buf.extension() {
        match ext.to_str().unwrap() {
            "ico" => {
                let mut file = File::open("images/favicon.ico")?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                bytes_handler(&buf, "image/x-icon", None).await
            }
            "svg" => {
                // build the response
                let xml = match file_name {
                    // "dev-badge.svg" => include_str!("assets/images/dev-badge.svg"), // for example
                    _ => "",
                };
                string_handler(xml, "image/svg+xml", None).await
            }
            _ => four_oh_four().await,
        }
    } else {
        four_oh_four().await
    }
}

/// Serve main page
pub async fn index(req: Request<Body>) -> HandlerResult {
    // Parse params, if any
    let params = form_urlencoded::parse(hyper::body::to_bytes(req).await?.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    // Parse sources
    let mut sources = {
        let mut ret = Vec::new();
        let all_possible = EventSource::all();
        for source in all_possible {
            if params.get(&source.markup_name()).is_some() {
                ret.push(*source);
            } else {
                ret.push(source.toggle());
            }
        }
        ret
    };

    // If none were checked, include everything
    let mut enabled_cnt = 0;
    for s in &sources {
        if s.enabled() {
            enabled_cnt += 1;
        }
    }
    if enabled_cnt == 0 {
        sources = EventSource::all().to_vec()
    }

    // Parse title search query
    let title_like = if let Some(s) = params.get("title") {
        if s.is_empty() {
            "%"
        } else {
            s
        }
    } else {
        "%"
    };

    // Grab connection
    let conn = DB_POOL.get()?;

    // Parse date search queries
    let (mut begin_date, mut end_date) = total_event_range(&conn)?;
    if let Some(s) = params.get("startdate") {
        if !s.is_empty() {
            begin_date = s.to_string();
        }
    }
    if let Some(s) = params.get("enddate") {
        if !s.is_empty() {
            end_date = s.to_string();
        }
    }

    // Request event set
    let events = filtered_events(&begin_date, &end_date, &sources, title_like, &conn)?;
    // Render template
    let last_refresh = if let Some(r) = latest_refresh(&conn)? {
        r.refresh_dt
    } else {
        "never".to_string()
    };
    let template = IndexTemplate::new(
        &begin_date,
        &end_date,
        events,
        title_like,
        &sources,
        &last_refresh,
    );
    let html = template.render()?;
    html_str_handler(&html).await
}

/// Serve 404 page
pub async fn four_oh_four() -> HandlerResult {
    let template = FourOhFourTemplate::default();
    let html = template.render()?;
    html_str_handler(&html).await
}

/// Request a re-scrape
pub async fn refresh_events() -> HandlerResult {
    // Only refresh if it's been more than 24h
    // If there's no refresh, we'll just continue on
    let conn = DB_POOL.get()?;
    if let Some(last_refresh) = latest_refresh(&conn)? {
        let now = Utc::now();
        let last = DateTime::parse_from_rfc3339(&last_refresh.refresh_dt)?;
        let duration = now.timestamp() - last.timestamp();
        if duration < (60 * 60 * 24) {
            // If it hasn't been 24 hours since the last scrape, do nothing
            return Ok(Response::default());
        }
    }
    let total_added = EventSource::scrape_all_events().await?;
    info!("Added {} new events", total_added);
    create_refresh(
        &conn,
        total_added.try_into().unwrap(), // I would be VERY surprised if we ever overflow an integer with this count
    )?;
    Ok(Response::default())
}
