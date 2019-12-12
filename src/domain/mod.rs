use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Clone)]
pub struct RepoName {
    pub owner: String,
    pub name: String,
}

impl RepoName {
    pub fn from<S: Into<String>>(input: S) -> Result<Self> {
        let t = input.into();
        let parts = t.split("/").collect::<Vec<_>>();

        if parts.len() < 2 {
            bail!("Could not derive owner and name from repo: {}", t);
        }

        let owner = String::from(parts[0]);
        let name = String::from(parts[1]);

        Result::Ok(RepoName { owner, name })
    }
}

#[derive(Serialize, Debug)]
pub struct Commit {
    pub branch: String,
    pub on: String,
    pub by: String,
    pub sha1: String,
    pub comment: String,
}

#[derive(Serialize)]
pub struct CommitsOnMaster {
    pub commits: u32,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize)]
pub struct Activity {
    pub master: CommitsOnMaster,
    pub prs: Vec<Item>,
    pub issues: Vec<Item>,
}

#[derive(Serialize)]
pub struct Repo {
    pub title: String,
    #[serde(rename = "lastCommit")]
    pub last_commit: Commit,
    pub activity: Activity,
}

pub mod sample {
    use super::*;

    pub fn last_commit() -> Commit {
        Commit {
            branch: "master".into(),
            on: "14min ago".into(),
            by: "felipesere".into(),
            sha1: "a11dfa26e15f4".into(),
            comment: "Add new questions".into(),
        }
    }

    pub fn data() -> Vec<super::Repo> {
        vec![
            Repo {
                title: "felipesere/advisor".into(),
                last_commit: Commit {
                    branch: "master".into(),
                    on: "14min ago".into(),
                    by: "felipesere".into(),
                    sha1: "a11dfa26e15f4".into(),
                    comment: "Add new questions".into(),
                },
                activity: Activity {
                    master: CommitsOnMaster { commits: 8 },
                    issues: vec![],
                    prs: vec![
                        Item {
                            link: "#".into(),
                            title: "Add new JSON backend for qeustions".into(),
                            by: "cgockel".into(),
                        },
                        Item {
                            link: "#".into(),
                            title: "Use FSUnit for testing".into(),
                            by: "fsere".into(),
                        },
                        Item {
                            link: "#".into(),
                            title: "Introduce external advisors".into(),
                            by: "cfereday".into(),
                        },
                    ],
                },
            },
            Repo {
                title: "async-rs/async-std".into(),
                last_commit: Commit {
                    branch: "master".into(),
                    on: "2 hours ago".into(),
                    by: "yoshwyut".into(),
                    sha1: "850b8ae9d06df".into(),
                    comment: "Merge pull request #344".into(),
                },
                activity: Activity {
                    master: CommitsOnMaster { commits: 34 },
                    prs: vec![
                        Item {
                            link: "#".into(),
                            title: "Implement DoubleEndedStream".into(),
                            by: "felipesere".into(),
                        },
                        Item {
                            link: "#".into(),
                            title: "Make channels faster".into(),
                            by: "stjepjang".into(),
                        },
                    ],
                    issues: vec![
                        Item {
                            link: "#".into(),
                            title: "Better support for byte ordered reads and writes?".into(),
                            by: "yoshwyut".into(),
                        },
                        Item {
                            link: "#".into(),
                            title: "Make errors more verbose".into(),
                            by: "zkat".into(),
                        },
                    ],
                },
            },
        ]
    }
}
