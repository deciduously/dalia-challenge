// Database handling

use super::*;
use chrono::prelude::*;
use diesel::{prelude::*, r2d2::ConnectionManager, sql_types::Bool, sqlite::SqliteConnection};
use diesel_migrations::*;
use lazy_static::lazy_static;
use std::ops::Deref;

const DEFAULT_DB_URL: &str = "db.sqlite";

lazy_static! {
    pub static ref DB_POOL: Pool =
        establish_and_run_migrations(DEFAULT_DB_URL).expect("Should create connection pool.");
}

/// R2D2 connection pool type
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Pooled connection
pub struct Conn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Conn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Connect to sqlite database and create r2d2 pool
pub fn establish_pool(url: &str) -> AppResult<Pool> {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    Ok(r2d2::Pool::new(manager)?)
}

/// Connect to sqlite database and run the migrations
fn establish_and_run_migrations(url: &str) -> AppResult<Pool> {
    let pool = establish_pool(url)?;
    embed_migrations!();
    embedded_migrations::run(&pool.get()?)?;
    Ok(pool)
}

/// Get all currently stored events
pub fn all_events(conn: &SqliteConnection) -> AppResult<Vec<Event>> {
    use schema::events::dsl::*;
    Ok(events.load::<Event>(conn)?)
}

/// Get a subset of events based on passed parameters
pub fn filtered_events(
    begin_date: &str,
    end_date: &str,
    src: &[EventSource],
    title_like: &str,
    conn: &SqliteConnection,
) -> AppResult<Vec<Event>> {
    use schema::events::dsl::*;

    // Start query builder
    let events = events;

    // Filter title
    let title_like_str = format!("%{}%", title_like);
    let filtered = events.filter(title.like(&title_like_str));

    // Filter sources
    let always_false = Box::new(source.eq(""));
    // Build compound query trait object from EventSource list
    let query: Box<dyn BoxableExpression<schema::events::table, _, SqlType = Bool>> = src
        .iter()
        .filter(|s| s.enabled())
        .map(|s| source.eq(s.as_str()))
        .fold(always_false, |query, item| Box::new(query.or(item)));

    // Return filtered result set ordered by date
    Ok(filtered
        .filter(query)
        .filter(event_date.between(begin_date, end_date))
        .order(event_date)
        .load::<Event>(conn)?)
}

/// Add a new event to the database
pub fn create_event(conn: &SqliteConnection, new_event: NewEvent) -> AppResult<usize> {
    Ok(diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)?)
}

/// Add a new refresh record
pub fn create_refresh(conn: &SqliteConnection, total_added: i32) -> AppResult<usize> {
    Ok(diesel::insert_into(refreshes::table)
        .values(NewRefresh {
            refresh_dt: &Utc::now().to_rfc3339(),
            total_added,
        })
        .execute(conn)?)
}

/// Get the most recent refresh, if any
pub fn latest_refresh(conn: &SqliteConnection) -> AppResult<Option<Refresh>> {
    use schema::refreshes::dsl::*;
    let res = refreshes.order(refresh_dt.desc()).limit(1).load::<Refresh>(conn)?;
    if res.is_empty() {
        Ok(None)
    } else {
        Ok(Some(res[0].clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    lazy_static! {
        pub static ref TEST_POOL: Pool =
            establish_and_run_migrations("test.sqlite").expect("Should establish test pool");
    }

    #[test]
    fn test_new_post() {
        let test = NewEvent::new(
            "Test Event",
            Some("It's not a real event".into()),
            "#",
            "Some really cool thing you don't want to miss",
            "2020-02-17",
            Some("2020-02-18".into()),
            EventSource::CoBerlin(true),
        );

        let conn = TEST_POOL.get().expect("Should get DB connection");

        assert_eq!(create_event(&conn, test).unwrap(), 1)
    }
}
