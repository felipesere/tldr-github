use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Error, Result};

use crate::db::{Db, FullStoredRepo, StoredRepo};
use crate::domain::{ItemKind, NewTrackedItem};

struct Thing {
    repo: StoredRepo,
    items: Vec<NewTrackedItem>,
}

struct InMemory {
    repos: Mutex<RefCell<HashMap<i32, Thing>>>,
    id: Mutex<i32>,
}

pub fn new() -> impl Db {
    InMemory {
        repos: Mutex::new(RefCell::new(HashMap::new())),
        id: Mutex::new(0),
    }
}

impl Db for InMemory {
    fn find_repo(&self, id: i32) -> Option<StoredRepo> {
        self.repos
            .lock()
            .expect("unable to lock in find_repo")
            .get_mut()
            // Hashmap from here downwards
            .get(&id)
            .map(|t| t.repo.clone())
    }

    fn insert_tracked_items(
        &self,
        repo: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<(), Error> {
        self.repos
            .lock()
            .expect("unable to lock in find_repo")
            .get_mut()
            // Hashmap from here downwards
            .entry(repo.id)
            .and_modify(|t| t.items.append(&mut items.clone()));

        Ok(())
    }

    fn update_tracked_item(&self, item: NewTrackedItem) -> Result<(), Error> {
        Ok(())
    }

    fn remove_tracked_item(&self, item: NewTrackedItem) -> Result<(), Error> {
        for (idx, v) in self
            .repos
            .lock()
            .unwrap()
            .get_mut()
            .values_mut()
            .enumerate()
        {
            let found = v.items.iter().find(|i| i.foreign_id == item.foreign_id);

            if found.is_some() {
                v.items.remove(idx);
            }
        }

        Ok(())
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>, Error> {
        let mut result = Vec::new();
        for thing in self
            .repos
            .lock()
            .expect("unable to lock repos")
            .get_mut()
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

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo, Error> {
        let mut id = self.id.lock().unwrap();

        let next = *id + 1;
        *id = next;
        let repo = StoredRepo::new(next, repo_name);

        self.repos.lock().unwrap().get_mut().insert(
            repo.id,
            Thing {
                repo: repo.clone(),
                items: Vec::new(),
            },
        );

        Ok(repo)
    }

    fn delete(&self, repo: i32) -> Result<(), Error> {
        self.repos.lock().unwrap().get_mut().remove(&repo);

        Ok(())
    }
}
