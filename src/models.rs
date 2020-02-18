// Rust types for DB records

use super::*;

#[derive(Debug, Clone, PartialEq, Queryable)]
pub struct Event {
    pub id: i32,
    pub href: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub synopsis: String,
    pub event_date: String,             // as ISO 8601 date string
    pub event_end_date: Option<String>, // TODO date type once working
    pub source: String,
}

#[derive(Debug, PartialEq, Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub href: &'a str,
    pub title: &'a str,
    pub subtitle: Option<String>,
    pub synopsis: &'a str,
    pub event_date: &'a str,
    pub event_end_date: Option<String>,
    pub source: &'a str,
}

impl<'a> PartialEq<NewEvent<'a>> for Event {
    fn eq(&self, rhs: &NewEvent) -> bool {
        self.href == rhs.href
            && self.title == rhs.title
            && self.synopsis == rhs.synopsis
            && self.event_date == rhs.event_date
            && self.source == rhs.source
    }
}

impl<'a> NewEvent<'a> {
    pub fn new(
        title: &'a str,
        subtitle: Option<String>,
        href: &'a str,
        synopsis: &'a str,
        event_date: &'a str,
        event_end_date: Option<String>,
        source: EventSource,
    ) -> Self {
        Self {
            href,
            title,
            subtitle,
            synopsis,
            event_date,
            event_end_date,
            source: source.as_str(),
        }
    }
}
