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

table! {
    refreshes (id) {
        id -> Integer,
        refresh_dt -> Text,
        total_added -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    refreshes,
);
