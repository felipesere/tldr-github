-- Your SQL goes here
CREATE TABLE issues
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    repo_id    INTEGER                           NOT NULL,
    title      TEXT                              NOT NULL,
    by         TEXT                              NOT NULL,
    link       TEXT                              NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (repo_id) REFERENCES repos (id)
);
