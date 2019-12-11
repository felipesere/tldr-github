use serde::Serialize;

#[derive(Serialize)]
pub struct Commit {
    branch: String,
    on: String,
    by: String,
    sha1: String,
    comment: String,
}

#[derive(Serialize)]
pub struct CommitsOnMaster {
    commits: u32,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub by: String,
}

#[derive(Serialize)]
pub struct Activity {
    master: CommitsOnMaster,
    prs: Vec<Item>,
    issues: Vec<Item>,
}

#[derive(Serialize)]
pub struct Repo {
    title: String,
    #[serde(rename = "lastCommit")]
    last_commit: Commit,
    activity: Activity,
}

pub mod sample {
    use super::*;

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
