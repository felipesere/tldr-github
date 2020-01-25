use anyhow::{bail, Result};
use async_std::prelude::*;
use async_std::task;
use chrono::{DateTime, Utc};
use futures::stream::futures_unordered::FuturesUnordered;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::db::{Db, StoredRepo};

pub mod api;

pub trait ClientForRepositories: Send + Sync {
    fn issue(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
    fn pull_request(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
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

#[derive(serde::Deserialize, Debug)]
pub enum ItemKind {
    #[serde(rename = "pr")]
    PR,
    #[serde(rename = "issue")]
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
    pub number: i32,
}

#[derive(Debug, Eq, PartialEq)]
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

pub fn add_new_repo(
    db: Arc<dyn Db>,
    _client: Arc<dyn ClientForRepositories>,
    maybe_name: String,
) -> Result<StoredRepo> {
    let name = RepoName::from(maybe_name)?;
    let repo = db.insert_new_repo(&name.to_string())?;

    Result::Ok(repo)
}

pub async fn retrieve_live_items(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    id: i32,
) -> Result<Vec<api::Item>> {
    if let Some(repo) = db.find_repo(id) {
        let name = repo.name();
        Result::Ok(
            client
                .entire_repo(&name)?
                .into_iter()
                .map(crate::domain::api::Item::from)
                .collect::<Vec<_>>(),
        )
    } else {
        bail!("did not find repo")
    }
}

pub async fn add_items_to_track(
    db: Arc<dyn Db>,
    client: Arc<dyn ClientForRepositories>,
    id: i32,
    items: Vec<api::ItemToTrack>,
) -> Result<()> {
    log::info!("We were about to add {:?} to {}", items, id);
    if let Some(repo) = db.find_repo(id) {
        let mut tasks = FuturesUnordered::new();
        for item in items {
            let name = repo.name();
            let c = client.clone();
            tasks.push(task::spawn(async move {
                match item.kind {
                    ItemKind::Issue => c.issue(&name, item.nr),
                    ItemKind::PR => c.pull_request(&name, item.nr),
                }
            }))
        }

        // TODO:  I am surprised I have to do this instead of tasks.collect().await
        let mut res = Vec::new();
        while let Some(i) = tasks.next().await {
            let inner = i?;
            res.push(inner);
        }

        db.insert_tracked_items(&repo, res)
    } else {
        bail!("Could not find repo {}", id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::{Db, FullStoredRepo, StoredRepo};
    use anyhow::Result;
    use async_std::task;
    use mockall::mock;

    mock!(
        pub Database { }
        trait Db {
            fn find_repo(&self, id: i32) -> Option<StoredRepo>;
            fn insert_tracked_items(
                &self,
                repo_name: &StoredRepo,
                items: Vec<NewTrackedItem>,
            ) -> Result<()>;
            fn all(&self) -> Result<Vec<FullStoredRepo>>;
            fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
            fn delete(&self, r: i32) -> Result<()>;
        }
    );

    mock!(
        pub Github{ }

        trait ClientForRepositories{
            fn issue(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
            fn pull_request(&self, repo: &RepoName, nr: i32) -> Result<NewTrackedItem>;
            fn entire_repo(&self, repo: &RepoName) -> Result<Vec<NewTrackedItem>>;
        }
    );

    #[test]
    fn does_not_add_items_to_a_non_existing_repo() {
        let mut db = MockDatabase::new();
        let github = MockGithub::new();

        db.expect_find_repo().times(1).returning(|_| None);
        let result = task::block_on(async move {
            add_items_to_track(Arc::new(db), Arc::new(github), 32, Vec::new()).await
        });

        assert!(result.is_err(), "should have failed to to add items");
    }

    #[test]
    #[ignore]
    fn queries_github_for_details_on_items_and_stores_them() {
        let mut db = MockDatabase::new();
        let github = MockGithub::new();

        db.expect_find_repo()
            .times(1)
            .returning(|_| Some(StoredRepo::new(1, "foo/bar")));

        let result = task::block_on(async move {
            add_items_to_track(Arc::new(db), Arc::new(github), 32, Vec::new()).await
        });

        assert!(result.is_err(), "should have failed to to add items");
    }
}
