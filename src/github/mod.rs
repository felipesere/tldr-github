use crate::domain;
use crate::BetterOption;
use anyhow::{bail, Result};
use async_std::task;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;
type GitObjectID = String;

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
    query_path = "graphql/broad-repo.graphql",
    response_derives = "Debug,Clone"
)]
pub struct BroadRepoView;

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
            let mut response = match surf::post("https://api.github.com/graphql") // TODO: this will need extracting or I'll need a second client for a fake backend?
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
                .unwrap_or(
                    domain::Author::new("ghost".into())
                        .with_link("https://github.com/ghost".into()),
                );

            items.push(domain::NewTrackedItem {
                foreign_id: pr.id,
                title: pr.title,
                link: pr.url,
                by: author,
                labels: labels,
                kind: domain::ItemKind::PR,
                last_updated: pr.updated_at,
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
                .unwrap_or(
                    domain::Author::new("ghost".into())
                        .with_link("https://github.com/ghost".into()),
                );

            items.push(domain::NewTrackedItem {
                foreign_id: issue.id,
                title: issue.title,
                link: issue.url,
                by: author,
                labels: labels,
                kind: domain::ItemKind::Issue,
                last_updated: issue.updated_at,
            })
        }

        return Result::Ok(items);
    }

    fn issues(&self, repo: &domain::RepoName) -> Result<Vec<domain::NewIssue>> {
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

            let labels: Vec<domain::Label> = funky_flatten(issue.labels.unwrap().nodes)
                .into_iter()
                .map(|s| domain::Label::new(s.name))
                .collect();

            let item = domain::NewIssue {
                by: domain::Author::new(issue.author.unwrap().login),
                link: issue.url,
                title: issue.title,
                labels,
            };

            items.push(item)
        }

        log::info!("found issues {} on Github", items.len());

        Result::Ok(items)
    }

    fn pull_requests(&self, repo: &domain::RepoName) -> Result<Vec<domain::NewPullRequest>> {
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

            let labels: Vec<domain::Label> = funky_flatten(pr.labels.unwrap().nodes)
                .into_iter()
                .map(|s| domain::Label::new(s.name))
                .collect();

            let author = pr.author.unwrap();

            let item = domain::NewPullRequest {
                by: domain::Author::new(author.login).with_link(author.url),
                link: pr.url,
                title: pr.title,
                labels,
            };

            items.push(item)
        }

        Result::Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ClientForRepositories;

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
    fn broad_repo_view() {
        let client = GithubClient::new("<< token >>");
        let repo = domain::RepoName::from("felipesere/advisorex").unwrap();

        let entire_repo = client
            .entire_repo(&repo)
            .expect("should be able to get PRs");

        assert_eq!(entire_repo.len(), 11);
    }
}
