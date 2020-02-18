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
    title_like: &'a str,
    sources: &'a [EventSource],
}

impl<'a> IndexTemplate<'a> {
    pub fn new(events: Vec<Event>, title_like: &'a str, sources: &'a [EventSource]) -> Self {
        Self {
            events,
            title_like: if title_like == "%" { "" } else { title_like },
            sources,
        }
    }
}

impl<'a> Default for IndexTemplate<'a> {
    fn default() -> Self {
        let conn = DB_POOL.get().expect("Should open database connection");
        Self {
            events: all_events(&conn).expect("Should retrieve all events"),
            title_like: "",
            sources: EventSource::all(),
        }
    }
}
