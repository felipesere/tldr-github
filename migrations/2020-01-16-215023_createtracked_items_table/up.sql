-- Your SQL goes here
CREATE TABLE tracked_items
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    repo_id    INTEGER                           NOT NULL,
    foreign_id TEXT                              NOT NULL,
    title      TEXT                              NOT NULL,
    by         TEXT                              NOT NULL,
    link       TEXT                              NOT NULL,
    labels     TEXT                              NOT NULL,
    kind       TEXT                              NOT NULL,
    last_updated TIMESTAMP                       NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repo_id) REFERENCES repos (id)
);
