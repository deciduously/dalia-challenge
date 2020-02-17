-- Your SQL goes here
CREATE TABLE events (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    title TEXT NOT NULL,
    synopsis TEXT NOT NULL,
    event_date TEXT NOT NULL,
    source TEXT NOT NULL
)