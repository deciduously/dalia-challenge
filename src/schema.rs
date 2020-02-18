table! {
    events (id) {
        id -> Integer,
        href -> Text,
        title -> Text,
        subtitle -> Nullable<Text>,
        synopsis -> Text,
        event_date -> Text,
        event_end_date -> Nullable<Text>,
        source -> Text,
    }
}
