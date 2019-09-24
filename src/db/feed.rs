use crate::{common::error::Error, create_error, DB_CLIENT};

use super::{model::Feed, DB_NAME};

use std::option::Option;

use log::*;
use mongodb::ThreadedClient;
use wither::prelude::*;

const SCOPE: &str = "database/feed";

pub fn create_new_feed(model: Feed) -> Result<Feed, Error> {
    debug!("create_new_feed requested with model: {:?}", model);

    let client = &DB_CLIENT;
    let db = client.db(DB_NAME);

    debug!("creating feed from model data");
    let mut feed: Feed = Feed::new(
        model.title.as_str(),
        model.description.as_str(),
        model.link.as_str(),
    )?;

    match feed.save(db.clone(), Option::None) {
        Ok(_) => {
            debug!("successfully saved feed with uuid {:?} in the database", feed);
            Result::Ok(feed)
        },
        Err(e) => {
            warn!("could not save feed: {:?} | in the database: {:?}", feed, e);
            Err(create_error!(SCOPE, "error occurred when saving the feed in the database"))
        }
    }
}
