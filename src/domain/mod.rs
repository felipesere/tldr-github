use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

use crate::db::{Db, NewRepoEvent, RepoEvents, StoredRepo};
use std::sync::Arc;

pub mod api;

pub trait ClientForRepositories {
    fn issues(&self, repo: &RepoName) -> Result<Vec<NewIssue>>;
    fn pull_requests(&self, repo: &RepoName) -> Result<Vec<NewPullRequest>>;
    fn last_commit(&self, repo: &RepoName) -> Result<Commit>;
}

#[derive(Clone)]
pub struct RepoName {
    pub owner: String,
    pub name: String,
}

impl RepoName {
    pub fn from<S: Into<String>>(input: S) -> Result<Self> {
        let t = input.into();
        let parts = t.split('/').collect::<Vec<_>>();

        if parts.len() < 2 {
            bail!("Could not derive owner and name from repo: {}", t);
        }

        let owner = String::from(parts[0]);
        let name = String::from(parts[1]);

        Result::Ok(RepoName { owner, name })
    }
}

impl Display for RepoName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

/// used for inserting
#[derive(Debug)]
pub struct NewPullRequest {
    pub title: String,
    pub link: String,
    pub by: String,
}

/// used for inserting
#[derive(Debug)]
pub struct NewIssue {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub branch: String,
    pub on: DateTime<Utc>,
    pub by: String,
    pub sha1: String,
    pub comment: String,
}

pub fn add_new_repo(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    name: RepoName,
) -> anyhow::Result<StoredRepo> {
    let pulls = client.pull_requests(&name).unwrap_or_default();
    let issues = client.issues(&name).unwrap_or_default();
    let last_commit = client.last_commit(&name);

    let repo = db.insert_new_repo(&name.to_string())?;
    db.insert_prs(&repo, pulls)?;
    db.insert_issues(&repo, issues)?;

    last_commit
        .and_then(|commit| {
            db.insert_new_repo_activity(
                &repo,
                NewRepoEvent {
                    event: RepoEvents::LatestCommitOnMaster(commit),
                },
            )
        })
        .map(|_s| repo)
}
