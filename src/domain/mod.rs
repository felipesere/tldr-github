use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

use crate::db::{Db, StoredRepo};
use std::sync::Arc;

pub mod api;

pub trait ClientForRepositories {
    fn issues(&self, repo: &RepoName) -> Result<Vec<NewIssue>>;
    fn pull_requests(&self, repo: &RepoName) -> Result<Vec<NewPullRequest>>;
    fn entire_repo(&self, repo: &RepoName) -> Result<Vec<NewTrackedItem>>;
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

#[derive(Debug)]
pub enum ItemKind {
    PR,
    Issue,
}

impl ToString for ItemKind {
    fn to_string(&self) -> String {
        use ItemKind::*;
        match self {
            PR => String::from("pr"),
            Issue => String::from("issue"),
        }
    }
}

#[derive(Debug)]
pub struct NewTrackedItem {
    pub title: String,
    pub link: String,
    pub by: Author,
    pub labels: Vec<Label>,
    pub kind: ItemKind,
    pub foreign_id: String,
    pub last_updated: DateTime<Utc>,
}

/// used for inserting
#[derive(Debug)]
pub struct NewPullRequest {
    pub title: String,
    pub link: String,
    pub by: Author,
    pub labels: Vec<Label>,
}

/// used for inserting
#[derive(Debug)]
pub struct NewIssue {
    pub title: String,
    pub link: String,
    pub by: Author,
    pub labels: Vec<Label>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Label(String);

impl Label {
    pub fn new(name: String) -> Self {
        Label(name)
    }
}

impl<T: Into<String>> From<T> for Label {
    fn from(val: T) -> Self {
        Label::new(val.into())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    link: Option<String>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Author { name, link: None }
    }

    pub fn with_link(mut self, url: String) -> Self {
        self.link = Some(url);
        self
    }
}

impl<T: Into<String>> From<T> for Author {
    fn from(val: T) -> Self {
        Author::new(val.into())
    }
}

pub fn add_new_repo(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    name: RepoName,
) -> Result<StoredRepo> {
    let items = client.entire_repo(&name)?;

    let repo = db.insert_new_repo(&name.to_string())?;
    db.insert_tracked_items(&repo, items)?;

    Result::Ok(repo)
}
