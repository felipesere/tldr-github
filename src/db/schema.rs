table! {
    repos (id) {
        id -> Integer,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tracked_items (id) {
        id -> Integer,
        repo_id -> Integer,
        foreign_id -> Text,
        title -> Text,
        by -> Text,
        link -> Text,
        labels -> Text,
        kind -> Text,
        last_updated -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(tracked_items -> repos (repo_id));

allow_tables_to_appear_in_same_query!(
    repos,
    tracked_items,
);
