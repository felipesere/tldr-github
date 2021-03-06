use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::db::FullStoredRepo;
use crate::domain::{ItemKind, Label, NewTrackedItem};

#[derive(serde::Deserialize, Debug)]
pub struct AddNewRepo {
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ItemToTrack {
    pub kind: ItemKind,
    pub nr: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct AddTrackedItemsForRepo {
    pub items: Vec<ItemToTrack>,
}

impl From<FullStoredRepo> for Repo {
    fn from(other: FullStoredRepo) -> Self {
        let FullStoredRepo {
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

impl From<NewTrackedItem> for Item {
    fn from(other: NewTrackedItem) -> Self {
        Item {
            nr: other.number,
            kind: other.kind.to_string(),
            title: other.title,
            link: other.link,
            by: other.by.name,
            labels: Label::expose(&other.labels),
            last_updated: other.last_updated.to_rfc3339(),
        }
    }
}

// Used in the JSON store
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub nr: i32,
    pub kind: String,
    pub title: String,
    pub link: String,
    pub by: String,
    pub labels: Vec<String>,
    pub last_updated: String,
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

#[cfg(test)]
mod test {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[test]
    fn serialize_add_repo_json() {
        let data = r#"
        {
            "name": "foo/bar"
        }
        "#;

        let add: AddNewRepo = serde_json::from_str(data).unwrap();

        assert_eq!(add.name, "foo/bar".to_owned())
    }

    #[test]
    fn serialize_adding_items_to_track_json() {
        let data = r#"
        {
          "items": [
            {
              "kind": "issue",
              "nr": 32
            },
            {
              "kind": "pr",
              "nr": 11
            }
          ]
        }
        "#;

        let add: AddTrackedItemsForRepo = serde_json::from_str(data).unwrap();

        assert_eq!(add.items.len(), 2)
    }

    #[test]
    fn serialize_an_entire_repo_json() {
        let repo = Repo {
            id: 42,
            title: "foo/bar".into(),
            activity: Activity {
                prs: vec![Item {
                    kind: ItemKind::PR.to_string(),
                    nr: 1,
                    title: "Fix important build failure".into(),
                    link: "https://example.com/1".into(),
                    by: "Someone".into(),
                    labels: vec!["foo".to_string(), "bar".to_string()],
                    last_updated: "2019-09-18T01:24:29+00:00".to_string(),
                }],
                issues: vec![Item {
                    kind: ItemKind::Issue.to_string(),
                    nr: 10,
                    title: "Important".into(),
                    link: "https://example.com/1".into(),
                    by: "Someone".into(),
                    labels: vec!["foo".to_string()],
                    last_updated: "2019-09-18T01:24:29+00:00".to_string(),
                }],
            },
        };

        let repo_json = serde_json::to_value(&repo).unwrap();

        assert_json_eq!(
            repo_json,
            json!({
              "id": 42,
              "title": "foo/bar",
              "activity": {
                "prs": [
                  {
                    "nr": 1,
                    "kind": "pr",
                    "title": "Fix important build failure",
                    "link": "https://example.com/1",
                    "by": "Someone",
                    "last_updated": "2019-09-18T01:24:29+00:00",
                    "labels": [
                      "foo",
                      "bar"
                    ]
                  }
                ],
                "issues": [
                  {
                    "nr": 10,
                    "kind": "issue",
                    "title": "Important",
                    "link": "https://example.com/1",
                    "by": "Someone",
                    "last_updated": "2019-09-18T01:24:29+00:00",
                    "labels": [
                      "foo"
                    ]
                  }
                ]
              }
            })
        );
    }
}
