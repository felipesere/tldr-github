mod schema;

use anyhow::{bail, Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

use crate::domain::*;

use schema::issues;
use schema::pull_requests;
use schema::repos;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

#[derive(Debug, Queryable)]
pub struct StoredRepo {
    pub id: i32,
    pub title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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

#[derive(Debug, Queryable)]
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

#[derive(Debug, Queryable)]
pub struct StoredIssue {
    id: i32,
    repo_id: i32,
    pub title: String,
    pub by: String,
    pub link: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub fn delete(conn: &Conn, r: i32) -> Result<()> {
    use schema::repos::dsl::{repos, id};
    match diesel::delete(repos.filter(id.eq(r))).execute(conn) {
        Ok(size) if size == 1 => {},
        Ok(_) => bail!("{} not found", r),
        Err(m) => bail!("could not delete repo: {}", m),
    };

    use schema::pull_requests::dsl::{pull_requests, repo_id};
    match diesel::delete(pull_requests.filter(repo_id.eq(r))).execute(conn) {
        Ok(_) => {},
        Err(m) => bail!("could not delete prs for repo repo: {}", m),
    };

    use schema::issues::dsl::{issues, repo_id as issue_repo_id};
    match diesel::delete(issues.filter(issue_repo_id.eq(r))).execute(conn) {
        Ok(_) => {},
        Err(m) => bail!("could not delete issues for repo repo: {}", m),
    };

    Ok(())
}

pub fn insert_new_pr(conn: &Conn, repo: &StoredRepo, pr: &NewPullRequest) -> Result<StoredPullRequest> {
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

pub fn insert_prs(conn: &Conn, repo: &StoredRepo, prs: Vec<NewPullRequest>) -> Result<Vec<StoredPullRequest>> {
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

pub fn insert_issues(conn: &Conn, repo: &StoredRepo, issues: Vec<NewIssue>) -> Result<Vec<StoredIssue>> {
    issues.iter().map(|issue| insert_new_issue(conn, repo, issue)).collect()
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
            .first(conn)
            .with_context(|| "retrieving stored repo")
    })
}

pub fn all_repos(conn: &Conn) -> Result<Vec<StoredRepo>> {
    use schema::repos::dsl::*;
    repos.load(conn).with_context(|| "getting all repos")
}

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
    use super::*;
    use crate::config::DatabaseConfig;
    use crate::domain::*;

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
}
