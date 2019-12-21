use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub branch: String,
    pub on: String,
    pub by: String,
    pub sha1: String,
    pub comment: String,
}

#[derive(Serialize, Debug)]
pub struct CommitsOnMaster {
    pub commits: u32,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize, Debug)]
pub struct Activity {
    pub master: CommitsOnMaster,
    pub prs: Vec<Item>,
    pub issues: Vec<Item>,
}

#[derive(Serialize, Debug)]
pub struct Repo {
    pub id: i32,
    pub title: String,
    #[serde(rename = "lastCommit")]
    pub last_commit: Option<Commit>,
    pub activity: Activity,
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
