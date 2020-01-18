use serde::Serialize;
use std::convert::From;

impl From<crate::db::FullStoredRepo> for Repo {
    fn from(other: crate::db::FullStoredRepo) -> Self {
        let crate::db::FullStoredRepo {
            id,
            title,
            issues,
            prs,
            ..
        } = other;

        Repo {
            id,
            title,
            activity: Activity {
                issues: issues.into_iter().map(Item::from).collect(),
                prs: prs.into_iter().map(Item::from).collect(),
            },
        }
    }
}

impl From<crate::db::StoredPullRequest> for Item {
    fn from(other: crate::db::StoredPullRequest) -> Self {
        Item {
            nr: other.nr,
            title: other.title,
            link: other.link,
            by: other.by,
            labels: Vec::new(),
        }
    }
}

impl From<crate::db::StoredIssue> for Item {
    fn from(other: crate::db::StoredIssue) -> Self {
        Item {
            nr: other.nr,
            title: other.title,
            link: other.link,
            by: other.by,
            labels: Vec::new(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub nr: i32,
    pub title: String,
    pub link: String,
    pub by: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Activity {
    pub prs: Vec<Item>,
    pub issues: Vec<Item>,
}

#[derive(Serialize, Debug)]
pub struct Repo {
    pub id: i32,
    pub title: String,
    pub activity: Activity,
}
