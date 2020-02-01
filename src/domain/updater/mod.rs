use std::future::Future;
use std::sync::Arc;

use crate::db::FullStoredRepo;
use crate::domain::{ClientForRepositories, NewTrackedItem, RepoName, ItemKind};
use async_std::sync::Receiver;
use async_std::task;
use async_std::prelude::*;
use tracing::{event, Level};
use std::time::Duration;

pub struct Config {
    pub channel: Receiver<(RepoName, NewTrackedItem)>,
    pub client: Arc<dyn ClientForRepositories>,
}

pub fn start(config: Config) {
    task::spawn(async move {
        event!(Level::INFO, "starting to work on updates");
        let client = config.client;
        let mut inbound = config.channel.throttle(Duration::from_secs(1));
        while let Some((repo_name, item)) = inbound.next().await {
            let updated = match item.kind {
                ItemKind::PR => client.pull_request(&repo_name, item.number),
                ItemKind::Issue => client.issue(&repo_name, item.number),
            };

            if let Err(e) = updated {
                event!(Level::ERROR, "unable to get {} #{}: {}", repo_name, item.number, e);
                continue
            }

            let update = updated.unwrap();
            event!(Level::INFO, "Managed to update {} #{}", repo_name, update.number);
        }
    });
}
