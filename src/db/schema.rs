table! {
    issues (id) {
        id -> Integer,
        repo_id -> Integer,
        title -> Text,
        by -> Text,
        link -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    pull_requests (id) {
        id -> Integer,
        repo_id -> Integer,
        title -> Text,
        by -> Text,
        link -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    repo_activity_log (id) {
        id -> Integer,
        repo_id -> Integer,
        event -> Text,
        created_at -> Timestamp,
    }
}

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

joinable!(issues -> repos (repo_id));
joinable!(pull_requests -> repos (repo_id));
joinable!(repo_activity_log -> repos (repo_id));
joinable!(tracked_items -> repos (repo_id));

allow_tables_to_appear_in_same_query!(
    issues,
    pull_requests,
    repo_activity_log,
    repos,
    tracked_items,
);
