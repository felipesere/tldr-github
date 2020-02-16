use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::sync::Mutex;

use anyhow::Error;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::db::{Db, FullStoredRepo, StoredRepo};
use crate::domain::NewTrackedItem;

pub struct JsonStore {
    file: Mutex<RefCell<File>>,
}

fn new(file: File) -> impl Db {
    JsonStore { file: Mutex::new(RefCell::new(file)) }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrackedItem {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repo {
    id: i32,
    items: Vec<TrackedItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct All {
    repos: HashMap<String, Repo>,
    next_id: i32,
}

impl Db for JsonStore {
    fn find_repo(&self, id: i32) -> Option<StoredRepo> {
        let mut unlocked = self.file.lock().unwrap();
        let mut f = unlocked.get_mut();
        f.seek(std::io::SeekFrom::Start(0));

        /*
        let mut buf = String::new();
        f.read_to_string(&mut buf);

        println!("#: {}", buf);
        */

        let all: All = serde_json::from_reader(&mut f).unwrap();


        for (name, repo) in all.repos {
            if repo.id == id {
                return Some(
                    StoredRepo {
                        id: repo.id,
                        title: name.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    }
                );
            }
        }

        None
    }

    fn insert_tracked_items(&self, repo_name: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_tracked_item(&self, item: NewTrackedItem) -> Result<(), Error> {
        unimplemented!()
    }

    fn remove_tracked_item(&self, item: NewTrackedItem) -> Result<(), Error> {
        unimplemented!()
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>, Error> {
        unimplemented!()
    }

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo, Error> {
        let mut unlocked = self.file.lock().unwrap();
        let mut f = unlocked.get_mut();
        f.seek(std::io::SeekFrom::Start(0));

        let mut all: All = serde_json::from_reader(&mut f).unwrap();

        all.repos.insert(repo_name.to_owned(), Repo {
            id: all.next_id + 1,
            items: vec![]
        });

        all.next_id += 1;

        let result = StoredRepo {
            id: all.next_id,
            title: repo_name.to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        f.set_len(0);
        f.seek(std::io::SeekFrom::Start(0));
        serde_json::to_writer(f, &all).map(|()| result).map_err(|e| anyhow::anyhow!(e))
    }

    fn delete(&self, repo: i32) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use anyhow::Result;

    use tempfile::tempfile;

    use super::*;

    #[test]
    fn it_finds_a_repo_within_json() {
        let json = r#"
        {
          "repos": {
              "felipesere/tldr-github": {
                    "id": 42,
                    "items": []
              }
          },
          "next_id": 42
        }
        "#;

        let mut f = tempfile().expect("created file");
        f.write_all(json.as_bytes());

        let repo = new(f);

        let found = repo.find_repo(42);

        assert!(found.is_some());
    }

    #[test]
    fn it_adds_it_to_the_json() {
        let json = r#"
        {
          "next_id": 42,
          "repos": {
              "felipesere/tldr-github": {
                    "id": 42,
                    "items": []
              }
          }
        }
        "#;

        let mut f = tempfile().expect("created file");
        f.write_all(json.as_bytes());

        let repo = new(f);

        let foo = repo.insert_new_repo("foo/bar").expect("could not insert new repo");

        let found = repo.find_repo(foo.id);

        assert!(found.is_some());
    }
}
