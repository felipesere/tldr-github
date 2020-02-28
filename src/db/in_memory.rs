use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{bail, Result};

use crate::db::{Db, FullStoredRepo, StoredRepo};
use crate::domain::{ItemKind, NewTrackedItem};

struct RepoAndItems {
    repo: StoredRepo,
    items: Vec<NewTrackedItem>,
}

struct InMemory {
    repos: Mutex<RefCell<HashMap<String, RepoAndItems>>>,
    id: Mutex<i32>,
}

pub fn new() -> impl Db {
    InMemory {
        repos: Mutex::new(RefCell::new(HashMap::new())),
        id: Mutex::new(0),
    }
}

impl Db for InMemory {
    fn find_repo(&self, repo_name: &str) -> Option<StoredRepo> {
        self.repos
            .lock()
            .expect("unable to lock in find_repo")
            .get_mut()
            // Hashmap from here downwards
            .get(repo_name)
            .map(|t| t.repo.clone())
    }

    fn insert_tracked_items(&self, repo: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<()> {
        self.repos
            .lock()
            .expect("unable to lock in find_repo")
            .get_mut()
            // Hashmap from here downwards
            .entry(repo.title.clone())
            .and_modify(|t| t.items.append(&mut items.clone()));

        Ok(())
    }

    fn update_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        for (idx, v) in self
            .repos
            .lock()
            .unwrap()
            .get_mut()
            // Hashmap from here downwards
            .values_mut()
            .enumerate()
        {
            let found = v.items.iter().find(|i| i.foreign_id == item.foreign_id);

            if found.is_some() {
                v.items.remove(idx);
                v.items.push(item);

                return Ok(());
            }
        }

        bail!(
            "original with foreign id {} not found when updating",
            item.foreign_id
        )
    }

    fn remove_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        for (idx, v) in self
            .repos
            .lock()
            .unwrap()
            .get_mut()
            // Hashmap from here downwards
            .values_mut()
            .enumerate()
        {
            let found = v.items.iter().find(|i| i.foreign_id == item.foreign_id);

            if found.is_some() {
                v.items.remove(idx);
                return Ok(());
            }
        }

        bail!(
            "original with foreign id {} not found when removing",
            item.foreign_id
        )
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let mut result = Vec::new();
        for thing in self
            .repos
            .lock()
            .expect("unable to lock repos")
            .get_mut()
            // Hashmap from here downwards
            .values()
        {
            let (issues, prs) = thing
                .items
                .clone()
                .into_iter()
                .partition(|i| i.kind == ItemKind::Issue);

            let r = FullStoredRepo {
                id: thing.repo.id,
                title: thing.repo.title.clone(),
                issues,
                prs,
            };
            result.push(r)
        }
        Ok(result)
    }

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let mut id = self.id.lock().unwrap();

        let next = *id + 1;
        *id = next;
        let repo = StoredRepo::new(next, repo_name);

        self.repos.lock().unwrap().get_mut().insert(
            repo.title.clone(),
            RepoAndItems {
                repo: repo.clone(),
                items: Vec::new(),
            },
        );

        Ok(repo)
    }

    fn delete(&self, repo: StoredRepo) -> Result<()> {
        self.repos.lock().unwrap().get_mut().remove(&repo.title);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::behaves_like_a_db!(new);
}
