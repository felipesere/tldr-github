table! {
    repos (id) {
        /// Local id of the repo
        id -> Integer,

        /// Name of the repo as it is seen on GitHub
        title -> Text,

        /// Technical columns:
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    pull_requests (id) {
        /// Local id of the pull request
        id -> Integer,

        /// The repo this PR belongs to
        repo_id -> Integer,

        /// The title of the pr as seen on GitHub
        title -> Text,

        /// The person who opened the PR
        by -> Text,

        /// The link to the PR on Github
        link -> Text,

        /// Technical columns:
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    issues (id) {
        /// Local id of the issue
        id -> Integer,

        /// The repo this issue belongs to
        repo_id -> Integer,

        /// The title of the issue as seen on GitHub
        title -> Text,

        /// The person who opened the issue
        by -> Text,

        /// The link to the issue on Github
        link -> Text,

        /// Technical columns:
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    activity_log (id) {
        /// Id of the entry in the activity_log
        id -> Integer,

        /// The JSON blog representing the event
        event -> Text,

        repo_id -> Nullable<Integer>,
        pull_request_id -> Nullable<Integer>,
        issue_id -> Nullable<Integer>,

        /// Technical columns:
        created_at -> Timestamp,
    }
}
