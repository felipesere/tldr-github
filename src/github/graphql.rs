use graphql_client::GraphQLQuery;
use crate::domain;
use async_std::task;
use anyhow::{bail, Error, Result};

type DateTime = chrono::DateTime<chrono::Utc>;
type URI = String;
type GitObjectID = String;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/pull-requests.graphql",
    response_derives = "Debug",
)]
pub struct PullRequestsView;

pub struct GithubClient {
    token: String,
}

impl GithubClient {
    pub fn new<S: Into<String>>(token: S) -> Self {
        GithubClient {
            token: token.into(),
        }
    }

    pub fn pull_requests<S: Into<String>>(&self, repo: S) -> Result<Vec<domain::Item>> {
        let t = repo.into();
        let parts = t.split("/").collect::<Vec<_>>();

        if parts.len() < 2 {
            return Result::Err(Error::msg("Could not derive owner and name from repo"));
        }

        let owner = parts[0];
        let name = parts[1];
        let query = PullRequestsView::build_query(pull_requests_view::Variables{ owner: owner.into(), name: name.into(), });

        let data: pull_requests_view::ResponseData = task::block_on(async {
            let mut response = match surf::post("https://api.github.com/graphql")
                .set_header("Authorization", format!("Bearer {}", self.token))
                .body_json(&query)
                .unwrap().await {
                    Ok(r) => r,
                    Err(err) => bail!(err),
                };


            if response.status() != 200 {
                match response.body_string().await {
                    Ok(http_error) => bail!(format!("Did not get a positive response: {}", http_error)),
                    Err(e) => bail!(e),
                };
            };

            let inner: graphql_client::Response<pull_requests_view::ResponseData> = response.body_json().await?;

            if let Some(errors) = inner.errors {
                return Result::Err(Error::msg(errors[0].message.clone()));
            }

            match inner.data {
                Some(valuable_stuff) => Result::Ok(valuable_stuff),
                None => Result::Err(Error::msg("There was no data in response"))
            }
        })?;


        let mut items = Vec::new();
        for maybe_pr in data.repository.expect("repository not present").pull_requests.nodes.expect("nodes not present") {
            let pr = maybe_pr.unwrap();
            let item = domain::Item {
                by: pr.author.unwrap().login,
                link: pr.url,
                title: pr.title,
            };

            items.push(item)
        }

        Result::Ok(items)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = GithubClient::new("<< token >>");

        let data = client.pull_requests("felipesere/advisorex").expect("should be able to get PRs");
        let titles: Vec<String> = data.iter().map(|pr| pr.title.clone()).collect();
        let expected: Vec<String> = vec!["Advice notes".into(), "Bump js-yaml from 3.13.0 to 3.13.1 in /assets".into()];

        assert_eq!(titles, expected)
    }
}
