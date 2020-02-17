// Types go here

use std::fmt;

/// All the implemented event source calendars
#[derive(Debug, Clone, Copy)]
pub enum EventSource {
    CoBerlin,
    DeutscheOperBerlin,
    Gorki,
    Berghain,
}

impl EventSource {
    // TODO: Workaround - the more robust solution would be a std::borrow::Cow
    pub fn as_str(self) -> &'static str {
        use EventSource::*;
        match self {
            CoBerlin => "CoBerlin",
            DeutscheOperBerlin => "DeutscheOperBerlin",
            Gorki => "Gorki",
            Berghain => "Berghain",
        }
    }
    pub fn url(self) -> &'static str {
        use EventSource::*;
        match self {
            CoBerlin => "https://www.co-berlin.org/en/calender",
            DeutscheOperBerlin => "https://www.deutscheoperberlin.de/en_EN/calendar",
            Gorki => "https://gorki.de/en/programme/2018/08/all",
            Berghain => "http://berghain.de/events/",
        }
    }
}

// NOTE: Provides EventSource::to_string()
impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just pass the Debug impl through
        write!(f, "{:?}", self)
    }
}
