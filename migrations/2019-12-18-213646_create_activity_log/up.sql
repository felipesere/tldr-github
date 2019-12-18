-- Your SQL goes here
CREATE TABLE activity_log (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event TEXT NOT NULL,
  repo_id INTEGER,
  pull_request_id INTEGER,
  issue_id INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (repo_id) REFERENCES repos(id),
  FOREIGN KEY (pull_request_id) REFERENCES pull_requests(id),
  FOREIGN KEY (issue_id) REFERENCES issues(id)
);
