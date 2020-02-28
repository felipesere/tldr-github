use std::cell::RefCell;
use std::path::Path;
use std::sync::Mutex;

use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::{Db, FullStoredRepo, StoredRepo};
use crate::domain::{Author, ItemKind, Label, NewTrackedItem, State};
use crate::domain::api::Item;

pub struct JsonStore {
    backing_store: jfs::Store,
    next_id: Mutex<RefCell<i32>>,
}

pub fn new<P: AsRef<Path>>(path: P) -> impl Db {
    let mut config = jfs::Config::default();
    config.pretty = true;
    config.single = true;

    JsonStore {
        backing_store: jfs::Store::new_with_cfg(path, config).unwrap(),
        next_id: Mutex::new(RefCell::new(0)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    id: i32,
    title: String,
    items: Vec<Item>,
}

impl Db for JsonStore {
    fn find_repo(&self, repo_name: &str) -> Option<StoredRepo> {
        let r = self.backing_store.get::<Repo>(repo_name);

        r.map(|repo| StoredRepo {
            id: repo.id,
            title: repo.title,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
            .ok()
    }

    fn insert_tracked_items(
        &self,
        repo_name: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<(), Error> {
        let repo = self.backing_store.get::<Repo>(&repo_name.title);

        if repo.is_ok() {
            let mut repo = repo.unwrap();
            repo.items
                .append(&mut items.into_iter().map(Item::from).collect());
            self.backing_store.save_with_id(&repo, &repo_name.title).map(|_arg| ()).context("inserting tracked item")
        } else {
            Ok(())
        }
    }

    fn update_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<(), Error> {
        self.remove_tracked_item(repo, item.clone())
            .and_then(|_| self.insert_tracked_items(repo, vec![item]))
    }

    fn remove_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<(), Error> {
        let repo = self.backing_store.get::<Repo>(&repo.title);

        let target_nr = item.number.clone();

        if repo.is_ok() {
            let mut repo = repo.unwrap();

            repo.items.retain(|i| i.nr != target_nr);

            return self
                .backing_store
                .save_with_id(&repo, &repo.title)
                .map(|_f| ())
                .context("removing");
        }

        Ok(())
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>, Error> {
        let tree = self.backing_store.all::<Repo>();

        tree.map(|all| {
            all.into_iter()
                .map(|(title, repo)| {
                    let (issues, prs) = repo
                        .items
                        .clone()
                        .into_iter()
                        .map(|item| NewTrackedItem {
                            title: item.title.clone(),
                            state: State::Open, // TODO: Odd
                            link: item.link.clone(),
                            by: Author::new(item.by),
                            labels: Label::map(&item.labels[..]),
                            kind: item.kind.into(),
                            foreign_id: "1234".into(), // TOOD Odd...
                            last_updated: DateTime::parse_from_rfc3339(&item.last_updated)
                                .unwrap()
                                .with_timezone(&Utc),
                            number: item.nr,
                        })
                        .partition(|i| i.kind == ItemKind::Issue);

                    FullStoredRepo {
                        id: repo.id,
                        title,
                        issues,
                        prs,
                    }
                })
                .collect()
        })
            .context("getting all repos")
    }

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo, Error> {
        let mut locked = self.next_id.lock().unwrap();
        let next = locked.get_mut();
        let id = *next + 1;
        *next = id;

        self.backing_store.save_with_id(
            &Repo {
                id,
                title: repo_name.to_owned(),
                items: Vec::new(),
            },
            repo_name,
        )?;

        let repo = StoredRepo {
            id,
            title: repo_name.to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Ok(repo)
    }

    fn delete(&self, repo: StoredRepo) -> Result<(), Error> {
       self.backing_store
            .delete(&repo.title)
            .context("deleting a repo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file_name() -> String {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();
        format!("{}.json", rand_string)
    }

    fn setup() -> impl Db {
        let mut file_path = std::env::temp_dir();
        file_path.push(file_name());

        new(file_path)
    }

    crate::behaves_like_a_db!(setup);  
}
