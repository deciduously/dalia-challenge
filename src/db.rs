// Database handling

use crate::{models::*, schema};
use diesel::{prelude::*, r2d2::ConnectionManager, sqlite::SqliteConnection};
use diesel_migrations::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::{env, ops::Deref};

lazy_static! {
    pub static ref DB_POOL: Pool = {
        dotenv().ok();
        let pool = establish_pool(&env::var("DATABASE_URL").expect("Should find configured DATABASE_URL environment variable"));
        // Run migrations
        embed_migrations!();
        embedded_migrations::run(&pool.get().expect("Should connect to DB pool")).expect("Should create DB table");
        pool
    };
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

pub fn establish_pool(url: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::new(manager).unwrap_or_else(|_| panic!("Should create connection pool to {}", url))
}

pub fn all_events(conn: &SqliteConnection) -> Vec<Event> {
    use schema::events::dsl::*;
    events
        .limit(5)
        .load::<Event>(conn)
        .expect("Should load events from DB")
}

/// Add a new event to the database.  True on success, false on failure
pub fn create_event(conn: &SqliteConnection, new_event: NewEvent) -> bool {
    use schema::events;
    diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)
        .is_ok()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    lazy_static! {
        pub static ref TEST_POOL: Pool = {
            let pool = establish_pool("test.sqlite");
            // Run migrations
            embed_migrations!();
            embedded_migrations::run(&pool.get().expect("Should connect to DB pool")).expect("Should create DB table");
            pool
        };
    }

    #[test]
    fn test_new_post() {
        use crate::types::EventSource;
        let test = NewEvent::new(
            "Test Event",
            "Some really cool thing you don't want to miss",
            "2020-02-17",
            EventSource::CoBerlin,
        );

        let conn = TEST_POOL.get().expect("Should get DB connection");

        assert_eq!(create_event(&conn, test), true)
    }
}
