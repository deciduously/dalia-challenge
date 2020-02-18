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
    begin_date: &'a str,
    end_date: &'a str,
    events: Vec<Event>,
    title_like: &'a str,
    sources: &'a [EventSource],
}

impl<'a> IndexTemplate<'a> {
    pub fn new(
        begin_date: &'a str,
        end_date: &'a str,
        events: Vec<Event>,
        title_like: &'a str,
        sources: &'a [EventSource],
    ) -> Self {
        Self {
            begin_date,
            end_date,
            events,
            title_like: if title_like == "%" { "" } else { title_like },
            sources,
        }
    }
}
