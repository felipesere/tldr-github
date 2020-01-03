use serde::{Deserialize, Serialize};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub branch: String,
    pub on: String,
    pub by: String,
    pub sha1: String,
    pub comment: String,
}

impl From<crate::domain::Commit> for Commit {
    fn from(other: crate::domain::Commit) -> Self {
        let time_since_commit = other.on.signed_duration_since(Utc::now());

        let human = HumanTime::from(time_since_commit);

        Commit {
            branch: other.branch,
            on: human.to_text_en(Accuracy::Rough, Tense::Present),
            by: other.by,
            sha1: other.sha1,
            comment: other.comment,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CommitsOnMaster {
    pub commits: u32,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize, Debug)]
pub struct Activity {
    pub master: CommitsOnMaster,
    pub prs: Vec<Item>,
    pub issues: Vec<Item>,
}

#[derive(Serialize, Debug)]
pub struct Repo {
    pub id: i32,
    pub title: String,
    #[serde(rename = "lastCommit")]
    pub last_commit: Option<Commit>,
    pub activity: Activity,
}
