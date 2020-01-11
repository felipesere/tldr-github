use std::io::Write;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteConnection};

use schema::{issues, pull_requests, repo_activity_log, repos};

use crate::domain::{Commit, NewIssue, NewPullRequest};

mod schema;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

pub trait Db {
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
    fn insert_new_pr(&self, repo: &StoredRepo, pr: &NewPullRequest) -> Result<StoredPullRequest>;
    fn insert_prs(
        &self,
        repo: &StoredRepo,
        prs: Vec<NewPullRequest>,
    ) -> Result<Vec<StoredPullRequest>>;
    fn insert_new_issue(&self, repo: &StoredRepo, issue: &NewIssue) -> Result<StoredIssue>;
    fn insert_issues(&self, repo: &StoredRepo, issues: Vec<NewIssue>) -> Result<Vec<StoredIssue>>;
    fn insert_new_repo_activity(
        &self,
        repo: &StoredRepo,
        new_event: NewRepoEvent,
    ) -> Result<StoredRepoEvent>;
    fn find_last_activity_for_repo(&self, r: i32) -> Result<Option<StoredRepoEvent>>;
    fn find_prs_for_repo(&self, r: i32) -> Result<Vec<StoredPullRequest>>;
    fn find_issues_for_repo(&self, r: i32) -> Result<Vec<StoredIssue>>;
    fn all(&self) -> Result<Vec<FullStoredRepo>>;
    fn delete(&self, r: i32) -> Result<()>;
}

pub struct SqliteDB {
    pub(crate) conn: Arc<SqlitePool>,
}

impl Db for SqliteDB {
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let conn = self.conn.get()?;
        insert_new_repo(&conn, repo_name)
    }
    fn insert_new_pr(&self, repo: &StoredRepo, pr: &NewPullRequest) -> Result<StoredPullRequest> {
        let conn = self.conn.get()?;
        insert_new_pr(&conn, repo, pr)
    }
    fn insert_prs(
        &self,
        repo: &StoredRepo,
        prs: Vec<NewPullRequest>,
    ) -> Result<Vec<StoredPullRequest>> {
        let conn = self.conn.get()?;
        insert_prs(&conn, repo, prs)
    }
    fn insert_new_issue(&self, repo: &StoredRepo, issue: &NewIssue) -> Result<StoredIssue> {
        let conn = self.conn.get()?;
        insert_new_issue(&conn, repo, issue)
    }
    fn insert_issues(&self, repo: &StoredRepo, issues: Vec<NewIssue>) -> Result<Vec<StoredIssue>> {
        let conn = self.conn.get()?;
        insert_issues(&conn, repo, issues)
    }
    fn insert_new_repo_activity(
        &self,
        repo: &StoredRepo,
        new_event: NewRepoEvent,
    ) -> Result<StoredRepoEvent> {
        let conn = self.conn.get()?;
        insert_new_repo_activity(&conn, repo, new_event)
    }
    fn find_last_activity_for_repo(&self, r: i32) -> Result<Option<StoredRepoEvent>> {
        let conn = self.conn.get()?;
        Result::Ok(find_last_activity_for_repo(&conn, r))
    }

    fn find_prs_for_repo(&self, r: i32) -> Result<Vec<StoredPullRequest>> {
        let conn = self.conn.get()?;
        find_prs_for_repo(&conn, r)
    }
    fn find_issues_for_repo(&self, r: i32) -> Result<Vec<StoredIssue>> {
        let conn = self.conn.get()?;
        find_issues_for_repo(&conn, r)
    }
    fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.get()?;
        all(&conn)
    }
    fn delete(&self, r: i32) -> Result<()> {
        let conn = self.conn.get()?;
        delete(&conn, r)
    }
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "repos"]
pub struct StoredRepo {
    pub id: i32,
    pub title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct FullStoredRepo {
    pub id: i32,
    pub title: String,
    pub issues: Vec<StoredIssue>,
    pub prs: Vec<StoredPullRequest>,
    pub events: Vec<StoredRepoEvent>,
}

impl FullStoredRepo {
    pub fn last_commit(&self) -> Option<Commit> {
        let mut last_commit = None;
        if let Some(existing_event) = self.events.first() {
            match existing_event.event {
                RepoEvents::LatestCommitOnMaster(ref c) => last_commit = Some(c.clone()),
            }
        };
        last_commit
    }
}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub title: &'a str,
}

#[derive(Insertable)]
#[table_name = "pull_requests"]
struct InsertPullRequest<'a> {
    repo_id: i32,
    title: &'a str,
    link: &'a str,
    by: &'a str,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[belongs_to(StoredRepo, foreign_key = "repo_id")]
#[table_name = "pull_requests"]
pub struct StoredPullRequest {
    id: i32,
    repo_id: i32,
    pub title: String,
    pub by: String,
    pub link: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "issues"]
pub struct InsertableIssue<'a> {
    pub repo_id: i32,
    pub title: &'a str,
    pub link: &'a str,
    pub by: &'a str,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[belongs_to(StoredRepo, foreign_key = "repo_id")]
#[table_name = "issues"]
pub struct StoredIssue {
    id: i32,
    repo_id: i32,
    pub title: String,
    pub by: String,
    pub link: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RepoEvents {
    LatestCommitOnMaster(Commit),
}

impl ToSql<Text, Sqlite> for RepoEvents {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        serde_json::to_writer(out, &self)
            .map(|()| serialize::IsNull::No)
            .map_err(|e| e.into())
    }
}

impl FromSql<Text, Sqlite> for RepoEvents {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        if let Some(data) = bytes {
            serde_json::from_slice(data.read_blob()).map_err(|e| e.into())
        } else {
            deserialize::Result::Err(anyhow::Error::msg("Unable to read RepoEvents from DB").into())
        }
    }
}

pub struct NewRepoEvent {
    pub event: RepoEvents,
}

#[derive(Insertable)]
#[table_name = "repo_activity_log"]
pub struct InsertableRepoEvent {
    repo_id: i32,
    event: RepoEvents,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[belongs_to(StoredRepo, foreign_key = "repo_id")]
#[table_name = "repo_activity_log"]
pub struct StoredRepoEvent {
    id: i32,
    repo_id: i32,
    pub event: RepoEvents,
    created_at: NaiveDateTime,
}

// this needs to be made transactional
pub fn delete(conn: &Conn, r: i32) -> Result<()> {
    match diesel::delete(repos::table.filter(repos::id.eq(r))).execute(conn) {
        Ok(size) if size == 1 => {}
        Ok(_) => bail!("{} not found", r),
        Err(m) => bail!("could not delete repo: {}", m),
    };

    match diesel::delete(pull_requests::table.filter(pull_requests::repo_id.eq(r))).execute(conn) {
        Ok(_) => {}
        Err(m) => bail!("could not delete prs for repo repo: {}", m),
    };

    match diesel::delete(issues::table.filter(issues::repo_id.eq(r))).execute(conn) {
        Ok(_) => {}
        Err(m) => bail!("could not delete issues for repo repo: {}", m),
    };

    match diesel::delete(repo_activity_log::table.filter(repo_activity_log::repo_id.eq(r)))
        .execute(conn)
    {
        Ok(_) => {}
        Err(m) => bail!("could not delete issues for repo repo: {}", m),
    };

    Ok(())
}

pub fn insert_new_pr(
    conn: &Conn,
    repo: &StoredRepo,
    pr: &NewPullRequest,
) -> Result<StoredPullRequest> {
    use schema::pull_requests::dsl::*;

    let insertable_pull_request = InsertPullRequest {
        repo_id: repo.id,
        title: &pr.title,
        link: &pr.link,
        by: &pr.by,
    };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(pull_requests)
            .values(&insertable_pull_request)
            .execute(conn)
            .with_context(|| format!("failed to insert {}", pr.title))?;

        // this is kinda meh, but there is no 'RETURNING'
        pull_requests
            .order(id.desc())
            .first(conn)
            .with_context(|| "retrieving stored pull request")
    })
}

pub fn insert_prs(
    conn: &Conn,
    repo: &StoredRepo,
    prs: Vec<NewPullRequest>,
) -> Result<Vec<StoredPullRequest>> {
    prs.iter().map(|pr| insert_new_pr(conn, repo, pr)).collect()
}

pub fn insert_new_issue(conn: &Conn, repo: &StoredRepo, issue: &NewIssue) -> Result<StoredIssue> {
    use schema::issues::dsl::*;

    let insertable_issue = InsertableIssue {
        repo_id: repo.id,
        title: &issue.title,
        link: &issue.link,
        by: &issue.by,
    };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(issues)
            .values(insertable_issue)
            .execute(conn)
            .with_context(|| format!("failed to insert {}", issue.title))?;

        // this is kinda meh, but there is no 'RETURNING'
        issues
            .order(id.desc())
            .first(conn)
            .with_context(|| "retrieving stored pull request")
    })
}

pub fn insert_issues(
    conn: &Conn,
    repo: &StoredRepo,
    issues: Vec<NewIssue>,
) -> Result<Vec<StoredIssue>> {
    issues
        .iter()
        .map(|issue| insert_new_issue(conn, repo, issue))
        .collect()
}

pub fn insert_new_repo_activity(
    conn: &Conn,
    repo: &StoredRepo,
    new_event: NewRepoEvent,
) -> Result<StoredRepoEvent> {
    use schema::repo_activity_log::dsl::*;

    let insertable_repo_event = InsertableRepoEvent {
        repo_id: repo.id,
        event: new_event.event,
    };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(repo_activity_log)
            .values(insertable_repo_event)
            .execute(conn)
            .map(|_| ())
            .map_err(anyhow::Error::new)?;

        repo_activity_log
            .order(id.desc())
            .first(conn)
            .with_context(|| "retrieving stored pull request")
    })
}

pub fn find_last_activity_for_repo(conn: &Conn, r: i32) -> Option<StoredRepoEvent> {
    use schema::repo_activity_log::dsl::*;

    repo_activity_log
        .filter(repo_id.eq(r))
        .order(created_at.desc())
        .first(conn)
        .ok()
}

pub fn insert_new_repo(conn: &Conn, repo_name: &str) -> Result<StoredRepo> {
    use schema::repos::dsl::*;
    let new_repo = NewRepo { title: repo_name };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(repos)
            .values(&new_repo)
            .execute(conn)
            .with_context(|| format!("failed to insert '{}'", repo_name))?;

        // this is kinda meh, but there is no 'RETURNING'
        repos
            .order(id.desc())
            .first::<StoredRepo>(conn)
            .with_context(|| "retrieving stored repo")
    })
}

pub fn all(conn: &Conn) -> Result<Vec<FullStoredRepo>> {
    use schema::repos::dsl::*;
    let rs: Vec<StoredRepo> = repos.load(conn).with_context(|| "getting all repos")?;

    let ids: Vec<i32> = rs.iter().map(|r| r.id).collect();

    use schema::issues::dsl::{issues, repo_id as issue_repo_id};
    let isx = issues
        .filter(issue_repo_id.eq_any(ids.clone()))
        .load::<StoredIssue>(conn)?
        .grouped_by(&rs[..]);

    use schema::pull_requests::dsl::{pull_requests, repo_id as pr_repo_id};
    let prs = pull_requests
        .filter(pr_repo_id.eq_any(ids.clone()))
        .load::<StoredPullRequest>(conn)?
        .grouped_by(&rs[..]);

    use schema::repo_activity_log::dsl::{repo_activity_log, repo_id as activity_repo_id};
    let activities = repo_activity_log
        .filter(activity_repo_id.eq_any(ids))
        .load::<StoredRepoEvent>(conn)?
        .grouped_by(&rs[..]);

    Result::Ok(
        rs.into_iter()
            .zip(isx)
            .zip(prs)
            .zip(activities)
            .map(|(((repo, issue), prs), events)| FullStoredRepo {
                id: repo.id,
                title: repo.title,
                issues: issue,
                prs,
                events,
            })
            .collect(),
    )
}

pub fn find_prs_for_repo(conn: &Conn, r: i32) -> Result<Vec<StoredPullRequest>> {
    use schema::pull_requests::dsl::*;
    pull_requests
        .filter(repo_id.eq(r))
        .load(conn)
        .with_context(|| "getting PRs for repo")
}

pub fn find_issues_for_repo(conn: &Conn, r: i32) -> Result<Vec<StoredIssue>> {
    use schema::issues::dsl::*;
    issues
        .filter(repo_id.eq(r))
        .load(conn)
        .with_context(|| "getting issues for repo")
}

#[cfg(test)]
mod test {
    use crate::config::DatabaseConfig;
    use crate::domain::*;
    use chrono::{TimeZone, Utc};

    use super::*;

    pub fn find_repo(conn: &Conn, n: i32) -> Option<StoredRepo> {
        use schema::repos::dsl::*;
        repos.find(n).first(conn).ok()
    }

    pub fn find_pr(conn: &Conn, n: i32) -> Option<StoredPullRequest> {
        use schema::pull_requests::dsl::*;
        pull_requests.find(n).first(conn).ok()
    }

    pub fn find_issue(conn: &Conn, n: i32) -> Option<StoredIssue> {
        use schema::issues::dsl::*;
        issues.find(n).first(conn).ok()
    }

    fn test_pool(
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> {
        let config = DatabaseConfig {
            file: ":memory:".into(),
            run_migrations: Some(true),
        };
        let pool = config.setup().expect("was not able to create test pool");

        pool.get().unwrap()
    }

    fn in_test_transaction<T, F>(conn: &Conn, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let mut user_result = None;

        let _ = conn.transaction::<(), _, _>(|| {
            user_result = Some(f());
            Err(diesel::result::Error::RollbackTransaction)
        });
        user_result.expect("this didn't work")
    }

    #[test]
    fn can_find_repos_it_just_stored() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            assert!(
                find_repo(&conn, repo.id).is_some(),
                "did not find stored repo"
            );

            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }

    #[test]
    fn can_find_pull_requests_it_just_stored() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let x = NewPullRequest {
                title: "Make the feature".into(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            let pr = insert_new_pr(&conn, &repo, &x)?;

            assert!(find_pr(&conn, pr.id).is_some(), "did not find stored PR");

            Result::<StoredPullRequest, anyhow::Error>::Ok(pr)
        })
        .unwrap();
    }

    #[test]
    fn can_find_all_prs_for_a_given_repo() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let title_x: String = "Make the feature".into();
            let x = NewPullRequest {
                title: title_x.clone(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            let title_y: String = "Make another feature".into();
            let y = NewPullRequest {
                title: title_y.clone(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            insert_prs(&conn, &repo, vec![x, y])?;

            let prs = find_prs_for_repo(&conn, repo.id).unwrap();

            let titles = prs.into_iter().map(|pr| pr.title).collect::<Vec<_>>();

            assert_eq!(titles, vec![title_x, title_y]);

            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }

    #[test]
    fn can_find_issues_it_just_stored() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let x = NewIssue {
                title: "Make the feature".into(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            let issue = insert_new_issue(&conn, &repo, &x)?;

            assert!(
                find_issue(&conn, issue.id).is_some(),
                "did not find stored issue"
            );

            Result::<StoredIssue, anyhow::Error>::Ok(issue)
        })
        .unwrap();
    }

    #[test]
    fn can_find_all_issues_for_a_given_repo() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let title_x: String = "Make the feature".into();
            let x = NewIssue {
                title: title_x.clone(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            let title_y: String = "Make another feature".into();
            let y = NewIssue {
                title: title_y.clone(),
                link: "http://example.com".into(),
                by: "Me".into(),
            };

            insert_new_issue(&conn, &repo, &x)?;
            insert_new_issue(&conn, &repo, &y)?;

            let issues = find_issues_for_repo(&conn, repo.id).unwrap();

            let titles = issues
                .into_iter()
                .map(|issue| issue.title)
                .collect::<Vec<_>>();

            assert_eq!(titles, vec![title_x, title_y]);

            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }

    #[test]
    fn can_add_new_activity_to_the_repo() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let event = NewRepoEvent {
                event: RepoEvents::LatestCommitOnMaster(Commit {
                    branch: "master".into(),
                    on: Utc.ymd(2019, 4, 22).and_hms(15, 37, 18),
                    by: "me".into(),
                    sha1: "kasdhfgasljdhf".into(),
                    comment: "This was a great commit".into(),
                }),
            };

            let stored_event = insert_new_repo_activity(&conn, &repo, event)?;

            match stored_event.event {
                RepoEvents::LatestCommitOnMaster(c) => assert_eq!(c.branch, "master".to_owned()),
                _ => assert!(false, "did mot get a 'LatestCommitOnMaster' event"),
            }
            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }
}
