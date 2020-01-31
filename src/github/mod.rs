use crate::domain;
use crate::BetterOption;
use anyhow::{bail, Result};
use async_std::task;
use graphql_client::GraphQLQuery;

use tracing::instrument;

type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/pull-request.graphql",
    response_derives = "Debug"
)]
pub struct PullRequestView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/issue.graphql",
    response_derives = "Debug"
)]
pub struct IssueView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/broad-repo.graphql",
    response_derives = "Debug,Clone"
)]
pub struct BroadRepoView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/repo-exists.graphql",
    response_derives = "Debug,Clone"
)]
pub struct RepoExistsView;

pub struct GithubClient {
    token: String,
}

impl GithubClient {
    pub fn new<S: Into<String>>(token: S) -> Self {
        GithubClient {
            token: token.into(),
        }
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

fn funky_flatten<T>(input: Option<Vec<Option<T>>>) -> Vec<T> {
    if input.is_none() {
        return Vec::new();
    }

    let input = input.unwrap();

    input
        .into_iter()
        .filter_map(|item| item)
        .collect::<Vec<T>>()
}

impl domain::ClientForRepositories for GithubClient {
    #[instrument(skip(self))]
    fn repo_exists(&self, repo: &domain::RepoName) -> Result<bool> {
        let query = RepoExistsView::build_query(repo_exists_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });

        match self.make_request::<graphql_client::QueryBody<repo_exists_view::Variables>, repo_exists_view::ResponseData>(query) {
            Ok(_) => Result::Ok(true),
            Err(_) => Result::Ok(false),
        }
    }

    fn entire_repo(&self, repo: &domain::RepoName) -> Result<Vec<domain::NewTrackedItem>> {
        let query = BroadRepoView::build_query(broad_repo_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
        });

        let data: broad_repo_view::ResponseData = self.make_request(query)?;
        let broad_repo_view::BroadRepoViewRepository {
            pull_requests,
            issues,
        } = data.repository.possibly("repository not present")?;

        let mut items = Vec::new();
        for maybe_pr in pull_requests.nodes.possibly("nodes not present")? {
            let pr = maybe_pr.possibly("no pr present")?;
            let labels = funky_flatten(pr.labels.possibly("no lables")?.nodes)
                .into_iter()
                .map(|s| domain::Label::new(s.name))
                .collect();

            let author = pr
                .author
                .map(|a| domain::Author::new(a.login).with_link(a.url))
                .unwrap_or(domain::Author::new("ghost").with_link("https://github.com/ghost"));


            let state = match pr.state {
                broad_repo_view::PullRequestState::OPEN => domain::State::Open,
                _ => domain::State::Closed,
            };

            items.push(domain::NewTrackedItem {
                state,
                foreign_id: pr.id,
                title: pr.title,
                link: pr.url,
                by: author,
                labels,
                kind: domain::ItemKind::PR,
                last_updated: pr.updated_at,
                number: pr.number as i32,
            })
        }

        for maybe_issue in issues.nodes.possibly("nodes not present")? {
            let issue = maybe_issue.possibly("no issue present")?;
            let labels = funky_flatten(issue.labels.possibly("no lables")?.nodes)
                .into_iter()
                .map(|s| domain::Label::new(s.name))
                .collect();

            let author = issue
                .author
                .map(|a| domain::Author::new(a.login).with_link(a.url))
                .unwrap_or(domain::Author::new("ghost").with_link("https://github.com/ghost"));

            let state = match issue.state {
                broad_repo_view::IssueState::OPEN => domain::State::Open,
                _ => domain::State::Closed,
            };

            items.push(domain::NewTrackedItem {
                state,
                foreign_id: issue.id,
                title: issue.title,
                link: issue.url,
                by: author,
                labels,
                kind: domain::ItemKind::Issue,
                last_updated: issue.updated_at,
                number: issue.number as i32,
            })
        }

        Result::Ok(items)
    }

    /// This will be used in the update-phase
    fn issue(&self, repo: &domain::RepoName, nr: i32) -> Result<domain::NewTrackedItem> {
        let query = IssueView::build_query(issue_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
            nr: nr as i64,
        });

        let data: issue_view::ResponseData = self.make_request(query)?;

        let issue = data
            .repository
            .possibly("no repository")?
            .issue
            .possibly("no issue")?;

        let labels = funky_flatten(issue.labels.possibly("no lables")?.nodes)
            .into_iter()
            .map(|s| domain::Label::new(s.name))
            .collect();

        let author = issue
            .author
            .map(|a| domain::Author::new(a.login).with_link(a.url))
            .unwrap_or(domain::Author::new("ghost").with_link("https://github.com/ghost"));

        let state = match issue.state {
            issue_view::IssueState::OPEN => domain::State::Open,
            _ => domain::State::Closed,
        };

        Result::Ok(domain::NewTrackedItem {
            state,
            foreign_id: issue.id,
            title: issue.title,
            link: issue.url,
            by: author,
            labels,
            kind: domain::ItemKind::Issue,
            last_updated: issue.updated_at,
            number: issue.number as i32,
        })
    }

    /// This will be used in the update-phase
    fn pull_request(&self, repo: &domain::RepoName, nr: i32) -> Result<domain::NewTrackedItem> {
        let query = PullRequestView::build_query(pull_request_view::Variables {
            owner: repo.owner.clone(),
            name: repo.name.clone(),
            nr: nr as i64,
        });

        let data: pull_request_view::ResponseData = self.make_request(query)?;

        let pr = data
            .repository
            .possibly("no repository")?
            .pull_request
            .possibly("no pull request")?;

        let labels = funky_flatten(pr.labels.possibly("no lables")?.nodes)
            .into_iter()
            .map(|s| domain::Label::new(s.name))
            .collect();

        let author = pr
            .author
            .map(|a| domain::Author::new(a.login).with_link(a.url))
            .unwrap_or(domain::Author::new("ghost").with_link("https://github.com/ghost"));

        let state = match pr.state {
            pull_request_view::PullRequestState::OPEN => domain::State::Open,
            _ => domain::State::Closed,
        };

        Result::Ok(domain::NewTrackedItem {
            state,
            foreign_id: pr.id,
            title: pr.title,
            link: pr.url,
            by: author,
            labels,
            kind: domain::ItemKind::PR,
            last_updated: pr.updated_at,
            number: pr.number as i32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ClientForRepositories, RepoName};

    #[test]
    fn grabs_pull_requests() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let pr = client
            .pull_request(&repo, 101)
            .expect("should be able to get PRs");

        assert_eq!(pr.title, "Advice notes".to_string());
    }

    #[test]
    fn issues_requests() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let issue = client
            .issue(&repo, 117)
            .expect("should be able to get issues");

        assert_eq!(issue.title, "Try out Github Actions".to_string());
    }

    #[test]
    fn repo_exist() {
        let client = GithubClient::new("<< token >>");

        let good_repo = RepoName::from("felipesere/advisorex").unwrap();
        let exists = client.repo_exists(&good_repo).unwrap();
        assert!(exists);

        let bad_repo = RepoName::from("felipesere/foo").unwrap();
        let not_exists = client.repo_exists(&bad_repo).unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn broad_repo_view() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let entire_repo = client
            .entire_repo(&repo)
            .expect("should be able to get PRs");

        assert_eq!(entire_repo.len(), 11);
    }
}
