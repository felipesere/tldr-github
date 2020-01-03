use crate::domain;
use anyhow::{bail, Result};
use async_std::task;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;
type GitObjectID = String;
type GitTimestamp = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/pull-requests.graphql",
    response_derives = "Debug"
)]
pub struct PullRequestsView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/issues.graphql",
    response_derives = "Debug"
)]
pub struct IssuesView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/last-commit.graphql",
    response_derives = "Debug,Clone"
)]
pub struct LastCommitView;

pub struct GithubClient {
    token: String,
}

impl GithubClient {
    pub fn new<S: Into<String>>(token: S) -> Self {
        GithubClient {
            token: token.into(),
        }
    }

    pub fn issues(&self, repo: &domain::RepoName) -> Result<Vec<domain::NewIssue>> {
        let query = IssuesView::build_query(issues_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });

        let data: issues_view::ResponseData = self.make_request(query)?;

        let mut items = Vec::new();
        for maybe_issue in data
            .repository
            .expect("repository not present")
            .issues
            .nodes
            .expect("nodes not present")
        {
            let issue = maybe_issue.unwrap();
            let item = domain::NewIssue {
                by: issue.author.unwrap().login,
                link: issue.url,
                title: issue.title,
            };

            items.push(item)
        }

        log::info!("found issues {} on Github", items.len());

        Result::Ok(items)
    }

    pub fn pull_requests(&self, repo: &domain::RepoName) -> Result<Vec<domain::NewPullRequest>> {
        let query = PullRequestsView::build_query(pull_requests_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });

        let data: pull_requests_view::ResponseData = self.make_request(query)?;

        let mut items = Vec::new();
        for maybe_pr in data
            .repository
            .expect("repository not present")
            .pull_requests
            .nodes
            .expect("nodes not present")
        {
            let pr = maybe_pr.unwrap();
            let item = domain::NewPullRequest {
                by: pr.author.unwrap().login,
                link: pr.url,
                title: pr.title,
            };

            items.push(item)
        }

        Result::Ok(items)
    }

    pub fn last_commit(&self, repo: &domain::RepoName) -> Result<domain::Commit> {
        let query = LastCommitView::build_query(last_commit_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });

        let data: last_commit_view::ResponseData = self.make_request(query)?;

        let commit  = match data.repository.expect("no repository").ref_.expect("no ref").target.on {
            crate::github::last_commit_view::LastCommitViewRepositoryRefTargetOn::Commit(c) => c,
            _ => bail!("unexpected variant"),
        };
        let real_commit = commit
            .history
            .edges
            .expect("no edges")
            .remove(0)
            .expect("no edge")
            .node
            .expect("there was no node");

        let author = real_commit.clone().author.expect("no author");
        let time_of_commit = author.date.unwrap();

        let result = domain::Commit {
            by: author.name.expect("no name"),
            comment: real_commit.clone().message_headline,
            on: time_of_commit,
            branch: "master".into(),
            sha1: real_commit.clone().oid,
        };

        Result::Ok(result)
    }

    fn make_request<Q: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        query: Q,
    ) -> Result<R> {
        task::block_on(async {
            let mut response = match surf::post("https://api.github.com/graphql")
                .set_header("Authorization", format!("Bearer {}", self.token))
                .body_json(&query)
                .unwrap()
                .await
            {
                Ok(r) => r,
                Err(err) => bail!(err),
            };

            if response.status() != 200 {
                match response.body_string().await {
                    Ok(http_error) => bail!("Did not get a positive response: {}", http_error),
                    Err(e) => bail!(e),
                };
            };

            let inner: graphql_client::Response<R> = response.body_json().await?;

            if let Some(errors) = inner.errors {
                bail!(errors[0].message.clone());
            }

            match inner.data {
                Some(valuable_stuff) => Result::Ok(valuable_stuff),
                None => bail!("There was no data in response"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grabs_pull_requests() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let data = client
            .pull_requests(&repo)
            .expect("should be able to get PRs");
        let titles: Vec<String> = data.iter().map(|pr| pr.title.clone()).collect();
        let expected: Vec<String> = vec![
            "Advice notes".into(),
            "Bump js-yaml from 3.13.0 to 3.13.1 in /assets".into(),
        ];

        assert_eq!(titles, expected)
    }

    #[test]
    fn issues_requests() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let data = client.issues(&repo).expect("should be able to get PRs");
        let titles: Vec<String> = data.iter().map(|pr| pr.title.clone()).take(3).collect();
        let expected: Vec<String> = vec![
            "Allow external feedback ".into(),
            "Allow the advisor to leave an unprompted note with advice".into(),
            "Upload custom image".into(),
        ];

        assert_eq!(titles, expected)
    }

    #[test]
    fn last_commit_on_master() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let commit = client
            .last_commit(&repo)
            .expect("should be able to get PRs");

        assert_eq!("a7f20cbde5fbf313a39e522859be5ffd04d0de80", &commit.sha1)
    }
}
