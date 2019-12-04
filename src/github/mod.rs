use serde::{Deserialize, Serialize};
use anyhow::Result;
use futures::future::TryFutureExt;

#[derive(Deserialize, Serialize)]
pub struct Label {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Person {
    login: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Issue {
    url: String,
    title: String,
    labels: Vec<Label>,
    body: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PullRequest {
    url: String,
    title: String,
    labels: Vec<Label>,
    body: Option<String>,
}

pub trait GithubClient {
    fn pulls(&self, repo_name: String) -> crate::util::BoxFuture<'static, Result<Vec<PullRequest>>>;
    fn issues(&self, repo_name: String) -> crate::util::BoxFuture<'static, Result<Vec<Issue>>>;
}

pub struct SurfOnGithub;

impl GithubClient for SurfOnGithub {
    fn pulls(&self, repo_name: String) -> crate::util::BoxFuture<'static, Result<Vec<PullRequest>>> {
            let uri = format!("https://api.github.com/repos/{}/pulls", repo_name);
            Box::pin(surf::get(uri).recv_json::<Vec<PullRequest>>().map_err(anyhow::Error::msg))
    }

    fn issues(&self, repo_name: String) -> crate::util::BoxFuture<'static, Result<Vec<Issue>>> {
            let uri = format!("https://api.github.com/repos/{}/issues", repo_name);
            Box::pin(surf::get(uri).recv_json::<Vec<Issue>>().map_err(anyhow::Error::msg))
    }
}


