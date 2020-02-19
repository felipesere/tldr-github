use std::fmt::{Display, Formatter};
use std::sync::Arc;

use anyhow::{bail, Result};
use async_std::prelude::*;
use async_std::task;
use chrono::{DateTime, Utc};
use futures::stream::futures_unordered::FuturesUnordered;
use tracing::{event, instrument, Level};

use crate::db::{Db, StoredRepo};

pub mod api;
pub mod updater;

pub trait ClientForRepositories: Send + Sync {
    fn repo_exists(&self, repo: &RepoName) -> Result<bool>;
    fn entire_repo(&self, repo: &RepoName) -> Result<Vec<NewTrackedItem>>;
    fn issue(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
    fn pull_request(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
}

#[derive(Clone, Debug)]
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

#[derive(serde::Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum ItemKind {
    #[serde(rename = "pr")]
    PR,
    #[serde(rename = "issue")]
    Issue,
}

impl From<String> for ItemKind {
    fn from(s: String) -> Self {
        if s == "pr" {
            return ItemKind::PR;
        }
        if s == "issue" {
            return ItemKind::Issue;
        }
        unreachable!()
    }
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum State {
    Open,
    Closed,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            State::Open => "open",
            State::Closed => "closed",
        };
        write!(f, "{}", val)
    }
}

// TODO: ths needs a better name
#[derive(Debug, Clone)]
pub struct NewTrackedItem {
    pub title: String,
    pub state: State,
    pub link: String,
    pub by: Author,
    pub labels: Vec<Label>,
    pub kind: ItemKind,
    pub foreign_id: String,
    pub last_updated: DateTime<Utc>,
    pub number: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Label(String);

impl Label {
    pub fn new(name: String) -> Self {
        Label(name)
    }

    pub fn join(labels: &[Label]) -> String {
        labels
            .iter()
            .map(|l| l.0.clone())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn split(raw: &str) -> Vec<Label> {
        if raw == "" {
            return Vec::new();
        }
        raw.split(',').map(|l| Label(l.to_owned())).collect()
    }

    pub fn map(raw: &[String]) -> Vec<Label> {
        raw.iter().map(|r| Label::new(r.to_owned())).collect()
    }

    pub fn expose(labels: &[Label]) -> Vec<String> {
        labels.iter().map(|l| l.0.clone()).collect()
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
    pub fn new<S: Into<String>>(name: S) -> Self {
        Author {
            name: name.into(),
            link: None,
        }
    }

    pub fn with_link<S: Into<String>>(mut self, url: S) -> Self {
        self.link = Some(url.into());
        self
    }
}

impl<T: Into<String>> From<T> for Author {
    fn from(val: T) -> Self {
        Author::new(val.into())
    }
}

#[instrument(skip(db, client))]
pub fn add_new_repo(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    maybe_name: String,
) -> Result<StoredRepo> {
    let name = RepoName::from(maybe_name)?;

    if client.repo_exists(&name)? {
        event!(
            Level::INFO,
            "{} found on Github proceeding to save to DB",
            &name
        );
        let repo = db.insert_new_repo(&name.to_string())?;
        event!(Level::INFO, "successfully save {} to DB", &name);
        Result::Ok(repo)
    } else {
        bail!("Repo {} not found on GitHub.", name.to_string())
    }
}

pub async fn retrieve_live_items(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    repo: StoredRepo,
) -> Result<Vec<api::Item>> {
    let name = repo.name();
    Result::Ok(
        client
            .entire_repo(&name)?
            .into_iter()
            .map(crate::domain::api::Item::from)
            .collect::<Vec<_>>(),
    )
}

pub async fn add_items_to_track(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    repo: StoredRepo,
    items: Vec<api::ItemToTrack>,
) -> Result<()> {
    let mut tasks = FuturesUnordered::new();
    for item in items {
        let name = repo.name();
        let c = client.clone();
        tasks.push(task::spawn(async move {
            match item.kind {
                ItemKind::Issue => c.issue(&name, item.nr),
                ItemKind::PR => c.pull_request(&name, item.nr),
            }
        }));

        // TODO:  I am surprised I have to do this instead of tasks.collect().await
        let mut res = Vec::new();
        while let Some(i) = tasks.next().await {
            let inner = i?;
            res.push(inner);
        }

        db.insert_tracked_items(&repo, res);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use async_std::task;
    use mockall::mock;

    use crate::db::{Db, FullStoredRepo, StoredRepo};

    use super::*;

    mock!(
        pub Database { }
        trait Db {
            fn find_repo(&self, repo_name: &str) -> Option<StoredRepo>;
            fn insert_tracked_items(
                &self,
                repo: &StoredRepo,
                items: Vec<NewTrackedItem>,
            ) -> Result<()>;
            fn update_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
            fn remove_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
            fn all(&self) -> Result<Vec<FullStoredRepo>>;
            fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
            fn delete(&self, repo: StoredRepo) -> Result<()>;
        }
    );

    mock!(
        pub Github{ }

        trait ClientForRepositories{
            fn repo_exists(&self, repo: &RepoName) -> Result<bool>;
            fn entire_repo(&self, repo: &RepoName) -> Result<Vec<NewTrackedItem>>;
            fn issue(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
            fn pull_request(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
        }
    );

    #[test]
    #[ignore]
    fn does_not_add_items_to_a_non_existing_repo() {
        let mut db = MockDatabase::new();
        let github = MockGithub::new();

        let repo = StoredRepo::new(32, "foo/bar");

        let result = task::block_on(async move {
            add_items_to_track(Arc::new(db), Arc::new(github), repo, Vec::new()).await
        });

        assert!(result.is_err(), "should have failed to to add items");
    }

    #[test]
    #[ignore]
    fn queries_github_for_details_on_items_and_stores_them() {
        let mut db = MockDatabase::new();
        let github = MockGithub::new();

        let repo = StoredRepo::new(32, "foo/bar");

        let result = task::block_on(async move {
            add_items_to_track(Arc::new(db), Arc::new(github), repo, Vec::new()).await
        });

        assert!(result.is_err(), "should have failed to to add items");
    }
}
