// Includes the scraping logic

use super::*;
use chrono::prelude::*;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use std::fmt;

/// Types that implement Calendar can be used to populate the event DB table
pub trait Calendar: Copy {
    /// Scrape all the events on the given page and add them to the database
    /// Returns number of events added
    fn scrape_events(self, document: Document) -> AppResult<usize>;
}

/// All the implemented event source calendars
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventSource {
    CoBerlin(bool),
    Berghain(bool),
}

impl EventSource {
    // Associated methods

    /// Get all the event sources to iterate over
    pub fn all() -> &'static [EventSource] {
        use EventSource::*;
        &[CoBerlin(true), Berghain(true)]
    }
    /// Scrape all the event sources, adding each new event found to the DB.  Returns number of events added
    pub async fn scrape_all_events() -> AppResult<usize> {
        let mut ret = 0;
        for src in Self::all() {
            let html = src.get_html().await?;
            let document = Document::from(html.as_str());
            ret += src.scrape_events(document)?;
        }
        Ok(ret)
    }

    // Instance methods

    pub fn as_str(self) -> &'static str {
        use EventSource::*;
        match self {
            CoBerlin(_) => "CoBerlin",
            Berghain(_) => "Berghain",
        }
    }
    /// Check whether this source is enabled
    pub fn enabled(self) -> bool {
        use EventSource::*;
        match self {
            CoBerlin(b) | Berghain(b) => b,
        }
    }
    /// Retrieve the current HTML from the source
    pub async fn get_html(self) -> AppResult<String> {
        let response = reqwest::get(&self.url_calendar()).await?;
        Ok(response.text().await?)
    }
    /// Name for use in HTML markup
    pub fn markup_name(self) -> String {
        format!("source-{}", self.as_str().to_lowercase())
    }
    /// Name for use in Display impl
    pub fn pretty_name(self) -> &'static str {
        use EventSource::*;
        match self {
            CoBerlin(_) => "C/O Berlin",
            Berghain(_) => self.as_str(),
        }
    }
    /// Static string for the webpage URL
    fn url_base(self) -> &'static str {
        use EventSource::*;
        match self {
            CoBerlin(_) => "http://www.co-berlin.org",
            Berghain(_) => "http://berghain.de",
        }
    }
    // Build a URL
    pub fn url(self, uri: &str) -> String {
        format!("{}/{}", self.url_base(), uri)
    }
    /// Static string for the webpage URL
    fn url_calendar(self) -> String {
        use EventSource::*;
        let uri = match self {
            CoBerlin(_) => "en/calender",
            Berghain(_) => "en/program",
        };
        self.url(uri)
    }
    /// Toggle from true to false or vice versa
    pub fn toggle(self) -> Self {
        use EventSource::*;
        match self {
            CoBerlin(b) => CoBerlin(!b),
            Berghain(b) => Berghain(!b),
        }
    }
}

impl Calendar for EventSource {
    fn scrape_events(self, document: Document) -> AppResult<usize> {
        // get all current events to search for matches
        let conn = DB_POOL.get()?;
        let all_events = all_events(&conn)?;

        // Iter through document
        use EventSource::*;
        let mut ret = 0;
        match self {
            CoBerlin(_) => {
                for node in
                    document.find(Class("seite-c-single").descendant(Class("calender-text")))
                {
                    let href = self.url(node.find(Name("a")).next().unwrap().attr("href").unwrap());
                    let (event_date, event_end_date) = {
                        let date = node
                            .find(Class("article-over-title"))
                            .next()
                            .unwrap()
                            .find(Class("article-date"))
                            .next()
                            .unwrap();
                        // range or single date?
                        match date.find(Class("date-display-range")).next() {
                            Some(div) => {
                                let begin =
                                    div.find(Class("date-display-start")).next().unwrap().text();
                                let end =
                                    div.find(Class("date-display-end")).next().unwrap().text();
                                let begin_dt = NaiveDate::parse_from_str(&begin, "%d/%m/%y")
                                    .expect("Should parse date");
                                let end_dt = NaiveDate::parse_from_str(&end, "%d/%m/%y")
                                    .expect("Should parse date");

                                (begin_dt.to_string(), Some(end_dt.to_string()))
                            }
                            None => {
                                let single_date = NaiveDate::parse_from_str(
                                    &date
                                        .find(Class("date-display-single"))
                                        .next()
                                        .unwrap()
                                        .text(),
                                    "%d/%m/%y",
                                )
                                .expect("Should parse date");
                                (single_date.to_string(), None)
                            }
                        }
                    };
                    let title = node.find(Class("article-title")).next().unwrap().text();
                    let subtitle = match node.find(Class("article-subtitle")).next() {
                        Some(s) => Some(s.text()),
                        None => None,
                    };
                    let synopsis = node.find(Class("article-text")).next().unwrap().text();

                    let new_event = NewEvent::new(
                        &title,
                        subtitle,
                        &href,
                        &synopsis,
                        &event_date,
                        event_end_date,
                        CoBerlin(true),
                    );

                    // Only add if it's a new event
                    let matches: Vec<Event> = all_events
                        .iter()
                        .filter(|el| **el == new_event)
                        .map(|el| el.to_owned())
                        .collect();

                    if matches.is_empty() {
                        ret += create_event(&conn, new_event)?;
                    }
                }
            }
            Berghain(_) => {
                for node in document.find(Class("upcoming-event")) {
                    let href = self.url(node.attr("href").unwrap());

                    let event_date = {
                        let mut date_node = node.find(Name("p"));
                        let mut node_text = date_node.next().unwrap().text();
                        node_text.retain(|c| c != '\n' && c != ' ');
                        let dt = NaiveDateTime::parse_from_str(&node_text, "%A%d.%m.%Ystart%R")?;

                        dt.to_string()
                    };

                    let title = node.find(Name("h2")).next().unwrap().text();
                    let subtitle = node.find(Name("h3")).next().unwrap().text();

                    let synopsis = {
                        let mut ret = String::new();
                        for child in node.find(Name("h4")) {
                            ret.push_str(&child.text());
                        }
                        ret
                    };

                    let new_event = NewEvent::new(
                        &title,
                        Some(subtitle),
                        &href,
                        &synopsis,
                        &event_date,
                        None,
                        Berghain(true),
                    );

                    // Only add if it's a new event
                    let matches: Vec<Event> = all_events
                        .iter()
                        .filter(|el| **el == new_event)
                        .map(|el| el.to_owned())
                        .collect();

                    if matches.is_empty() {
                        ret += create_event(&conn, new_event)?;
                    }
                }
            }
        }
        Ok(ret)
    }
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_name())
    }
}
