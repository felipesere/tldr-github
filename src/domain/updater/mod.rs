use std::sync::Arc;
use std::time::Duration;

use async_std::prelude::*;
use async_std::sync::Receiver;
use async_std::task;

use crate::db::{Db, StoredRepo};
use crate::domain::{ClientForRepositories, ItemKind, NewTrackedItem, State};

pub struct Config {
    pub channel: Receiver<(StoredRepo, NewTrackedItem)>,
    pub client: Arc<dyn ClientForRepositories>,
    pub db: Arc<dyn Db>,
}

pub fn start(config: Config) {
    task::spawn(async move {
        let db = config.db;
        let client = config.client;
        let mut inbound = config.channel.throttle(Duration::from_secs(1));
        while let Some((repo, item)) = inbound.next().await {
            let updated = match item.kind {
                ItemKind::PR => client.pull_request(&repo.name(), item.number),
                ItemKind::Issue => client.issue(&repo.name(), item.number),
            };

            if updated.is_err() {
                continue;
            }

            let updated = updated.unwrap();

            let result = match update(item, updated) {
                Outcome::Update(u) => db.update_tracked_item(&repo, u),
                Outcome::Remove(u) => db.remove_tracked_item(&repo, u),
                Outcome::Ignore => Result::Ok(()),
            };

            result.expect("it should have worked!")
        }
    });
}

pub enum Outcome {
    Update(NewTrackedItem),
    Ignore,
    Remove(NewTrackedItem),
}

pub fn update(old: NewTrackedItem, new: NewTrackedItem) -> Outcome {
    use Outcome::*;

    if new.state == State::Closed {
        return Remove(old);
    }

    if new.last_updated != old.last_updated {
        return Update(new);
    }

    Ignore
}
