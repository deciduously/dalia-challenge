// Rust types for DB records

use crate::{schema::events, types::*};

#[derive(Debug, Clone, PartialEq, Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub synopsis: String,
    pub event_date: String, // as ISO 8601 date string
    pub source: String,
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub title: &'a str,
    pub synopsis: &'a str,
    pub event_date: &'a str,
    pub source: &'a str,
}

impl<'a> NewEvent<'a> {
    pub fn new(
        title: &'a str,
        synopsis: &'a str,
        event_date: &'a str,
        source: EventSource,
    ) -> Self {
        Self {
            title,
            synopsis,
            event_date,
            source: source.as_str(),
        }
    }
}
