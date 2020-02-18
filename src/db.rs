// Database handling

use super::*;
use diesel::{prelude::*, r2d2::ConnectionManager, sqlite::SqliteConnection};
use diesel_migrations::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::{env, ops::Deref};

lazy_static! {
    pub static ref DB_POOL: Pool = {
        dotenv().ok();
        establish_and_run_migrations(
            &env::var("DATABASE_URL")
                .expect("Should find configured DATABASE_URL environment variable"),
        )
        .expect("Should create connection pool.")
    };
}

/// R2D2 connection pool type
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Pooled connection
// TODO this doesn't work?
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

pub fn all_events(conn: &SqliteConnection) -> AppResult<Vec<Event>> {
    use schema::events::dsl::*;
    Ok(events.load::<Event>(conn)?)
}

pub fn filtered_events(
    src: EventSource,
    title_like: &str,
    conn: &SqliteConnection,
) -> AppResult<Vec<Event>> {
    info!("title search: {}", title_like);
    // TODO search for source in all sources
    use schema::events::dsl::*;
    Ok(events
        .filter(
            source
                .like(&format!("%{}%", src.as_str()))
                .and(title.like(&format!("%{}%", title_like))),
        )
        .load::<Event>(conn)?)
}

/// Add a new event to the database.  True on success, false on failure
pub fn create_event(conn: &SqliteConnection, new_event: NewEvent) -> AppResult<usize> {
    Ok(diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)?)
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
            EventSource::CoBerlin,
        );

        let conn = TEST_POOL.get().expect("Should get DB connection");

        assert_eq!(create_event(&conn, test).unwrap(), 1)
    }
}
