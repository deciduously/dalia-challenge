// templates.rs
// Typed structs for each template in /templates/

use crate::{db::*, models::*};
use askama::Template;

#[derive(Default, Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {}

#[derive(Default, Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    events: Vec<Event>,
}

impl Default for IndexTemplate {
    fn default() -> Self {
        let conn = DB_POOL.get().expect("Should open database connection");
        Self {
            events: all_events(&conn),
        }
    }
}
