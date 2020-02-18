// templates.rs
// Typed structs for each template in /templates/

use super::*;
use askama::Template;

#[derive(Default, Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {}

#[derive(Default, Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    events: Vec<Event>,
    sources: &'a [EventSource],
}

impl<'a> IndexTemplate<'a> {
    pub fn new(events: Vec<Event>) -> Self {
        Self {
            events,
            sources: EventSource::all(),
        }
    }
}

impl<'a> Default for IndexTemplate<'a> {
    fn default() -> Self {
        let conn = DB_POOL.get().expect("Should open database connection");
        Self {
            events: all_events(&conn).expect("Should retrieve all events"),
            sources: EventSource::all(),
        }
    }
}
