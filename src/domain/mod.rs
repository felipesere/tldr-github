use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

use crate::db::{Db, SqliteDB, NewRepoEvent, RepoEvents, StoredRepo};
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
        let parts = t.split("/").collect::<Vec<_>>();

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub branch: String,
    pub on: DateTime<Utc>,
    pub by: String,
    pub sha1: String,
    pub comment: String,
}

pub struct Repo {
}

pub fn add_new_repo(
    db: Box<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    name: RepoName,
) -> anyhow::Result<StoredRepo> {
    let pulls = client.pull_requests(&name).unwrap_or(Vec::new());
    let issues = client.issues(&name).unwrap_or(Vec::new());
    let last_commit = client.last_commit(&name);

    let repo = db.insert_new_repo(&name.to_string())?;
    db.insert_prs(&repo, pulls)?;
    db.insert_issues(&repo, issues)?;

    last_commit.and_then(|commit| {
        db.insert_new_repo_activity(
            &repo,
            NewRepoEvent {
                event: RepoEvents::LatestCommitOnMaster(commit),
            })
    }).map(|_s| repo)
}

/*
pub fn get_all_repos(db: Box<dyn Db>) -> anyhow::Result<Vec<Repo>> {
    let repos = db.all_repos().unwrap();
    let mut result = Vec::new();
    for repo in repos {
        let pulls: Vec<domain::api::Item> = db.find_prs_for_repo(repo.id)
            .unwrap()
            .into_iter()
            .map(|pr| domain::api::Item {
                by: pr.by,
                title: pr.title,
                link: pr.link,
            })
            .collect();

        let issues: Vec<domain::api::Item> = db.find_issues_for_repo(repo.id)
            .unwrap()
            .into_iter()
            .map(|pr| domain::api::Item {
                by: pr.by,
                title: pr.title,
                link: pr.link,
            })
            .collect();

        let repo_event = db.find_last_activity_for_repo(repo.id);

        let mut last_commit = None;
        if let Some(existing_event) = repo_event {
            match existing_event.event {
                db::RepoEvents::LatestCommitOnMaster(c) => last_commit = Some(c),
            }
        }

        let r = domain::api::Repo {
            id: repo.id,
            title: repo.title,
            last_commit: last_commit.map(|c| domain::api::Commit::from(c)),
            activity: domain::api::Activity {
                master: domain::api::CommitsOnMaster { commits: 0 },
                prs: pulls,
                issues,
            },
        };

        result.push(r)
    }
    anyhow::Result::Ok(result)
}
*/
