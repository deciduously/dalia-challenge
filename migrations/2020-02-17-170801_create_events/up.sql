-- Your SQL goes here
CREATE TABLE events (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    href TEXT NOT NULL,
    title TEXT NOT NULL,
    subtitle TEXT,
    synopsis TEXT NOT NULL,
    event_date TEXT NOT NULL,
    event_end_date TEXT,
    source TEXT NOT NULL
)