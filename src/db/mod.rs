use async_trait::async_trait;
use std::sync::Arc;

use anyhow::Result;

// TODO: this needs to move away

use crate::domain::NewTrackedItem;

mod sqlite;

pub fn sqlite(database_url: &str, run_migrations: bool) -> Result<Arc<dyn Db>> {
    sqlite::new(database_url, run_migrations)
}

#[async_trait]
pub trait Db: Send + Sync {
    async fn find_repo(&self, repo_name: &str) -> Option<StoredRepo>;
    async fn insert_tracked_items(
        &self,
        repo: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<()>;
    async fn update_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
    async fn remove_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
    async fn all(&self) -> Result<Vec<FullStoredRepo>>;
    async fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
    async fn delete(&self, repo: StoredRepo) -> Result<()>;
}

#[derive(sqlx::FromRow)]
pub struct StoredRepo {
    pub id: i32,
    pub title: String,
}

impl StoredRepo {
    pub fn name(&self) -> crate::domain::RepoName {
        crate::domain::RepoName::from(&self.title).unwrap()
    }
}

#[derive(Debug)]
pub struct FullStoredRepo {
    pub id: i32,
    pub title: String,
    pub issues: Vec<NewTrackedItem>,
    pub prs: Vec<NewTrackedItem>,
}

impl FullStoredRepo {
    pub fn stored(&self) -> StoredRepo {
        StoredRepo {
            id: self.id,
            title: self.title.clone(),
            // TODO: get rid of some of these...
        }
    }

    pub fn items(&self) -> Vec<NewTrackedItem> {
        let mut res = Vec::new();

        res.append(&mut self.issues.clone());
        res.append(&mut self.prs.clone());

        res
    }
}

pub struct NewRepo<'a> {
    pub title: &'a str,
}

mod support {
    #[macro_export]
    macro_rules! behaves_like_a_db {
        ($setup_db:expr) => {
            use async_std::task;
            #[test]
            fn it_finds_a_repo_that_was_added() {
                let db = $setup_db();

                task::block_on(async move {
                    let repo1 = db.insert_new_repo("foo/bar").await.unwrap();
                    let repo2 = db.insert_new_repo("other").await.unwrap();

                    let found1 = db.find_repo(&repo1.title).await;
                    let found2 = db.find_repo(&repo2.title).await;

                    assert!(found1.is_some());
                    assert_eq!(found2.unwrap().title, "other")
                })
            }

            #[test]
            fn a_deleted_item_can_not_be_found() {
                let db = $setup_db();

                task::block_on(async move {
                    let repo = db.insert_new_repo("foo/baz").await.unwrap();

                    let title = repo.title.clone();
                    db.delete(repo).await.unwrap();

                    let found = db.find_repo(&title).await;

                    assert!(found.is_none());
                })
            }

            /*
                #[test]
                fn can_add_an_item_to_a_repo() {
                    use crate::domain::{Author, ItemKind, NewTrackedItem, State};
                    use chrono::Utc;

                    let db = $setup_db();

                    task::block_on(async move {
                        let repo = db.insert_new_repo("foo/bar").await.unwrap();

                        db.insert_tracked_items(
                            &repo,
                            vec![NewTrackedItem {
                                title: "some PR".to_string(),
                                state: State::Open,
                                link: "http://foo.bar".to_string(),
                                by: Author::new("Steve Hawking"),
                                labels: vec![],
                                kind: ItemKind::PR,
                                foreign_id: "sflhjsfklhjsd".to_string(),
                                last_updated: Utc::now(),
                                number: 1,
                            }],
                        )
                        .await
                        .expect("should have been able to insert tracked items");

                        let all = db.all().await.unwrap();

                        let found = all
                            .iter()
                            .find(|item| item.title == "foo/bar".to_string())
                            .unwrap();

                        assert_eq!(found.items().len(), 1);
                    })
                }

                #[test]
                fn can_update_an_added_item() {
                    use crate::domain::{Author, ItemKind, NewTrackedItem, State};
                    use chrono::Utc;
                    let db = $setup_db();

                    task::block_on(async move {
                        let repo = db.insert_new_repo("totally/madeup").await.unwrap();

                        db.insert_tracked_items(
                            &repo,
                            vec![NewTrackedItem {
                                title: "some PR".to_string(),
                                state: State::Open,
                                link: "http://foo.bar".to_string(),
                                by: Author::new("Steve Hawking"),
                                labels: vec![],
                                kind: ItemKind::PR,
                                foreign_id: "sflhjsfklhjsd".to_string(),
                                last_updated: Utc::now(),
                                number: 1,
                            }],
                        )
                        .await
                        .unwrap();

                        db.update_tracked_item(
                            &repo,
                            NewTrackedItem {
                                title: "changed-the-title".to_string(),
                                state: State::Open,
                                link: "http://foo.bar".to_string(),
                                by: Author::new("Steve Hawking"),
                                labels: vec![],
                                kind: ItemKind::PR,
                                foreign_id: "sflhjsfklhjsd".to_string(),
                                last_updated: Utc::now(),
                                number: 1,
                            },
                        )
                        .await
                        .unwrap();

                        let all = db.all().await.unwrap();

                        let matching_repo = all
                            .iter()
                            .find(|r| r.title == "totally/madeup".to_string())
                            .unwrap();

                        assert_eq!(matching_repo.items().len(), 1);
                        assert_eq!(
                            matching_repo.items()[0].title,
                            "changed-the-title".to_string()
                        )
                    })
                }

                #[test]
                fn an_added_tracked_item_can_be_removed() {
                    use crate::domain::{Author, ItemKind, NewTrackedItem, State};
                    use chrono::Utc;
                    let db = $setup_db();

                    task::block_on(async move {
                        let repo = db.insert_new_repo("abc/123").await.unwrap();
                        let tracked_item = NewTrackedItem {
                            title: "some PR".to_string(),
                            state: State::Open,
                            link: "http://foo.bar".to_string(),
                            by: Author::new("Steve Hawking"),
                            labels: vec![],
                            kind: ItemKind::PR,
                            foreign_id: "sflhjsfklhjsd".to_string(),
                            last_updated: Utc::now(),
                            number: 1,
                        };

                        db.insert_tracked_items(&repo, vec![tracked_item.clone()])
                            .await
                            .unwrap();

                        let all = db.all().await.unwrap();
                        let matching_repo = all.iter().find(|r| r.title == repo.title).unwrap();
                        assert_eq!(matching_repo.items().len(), 1);

                        db.remove_tracked_item(&repo, tracked_item).await.unwrap();

                        let all = db.all().await.unwrap();
                        let matching_repo = all.iter().find(|r| r.title == repo.title).unwrap();
                        assert_eq!(matching_repo.items().len(), 0);
                    })
                }
            */
        };
    }
}
