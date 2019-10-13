use crate::{
    common::{
        errors::{Error, FeedDbError},
        report::Report,
    },
    create_error, option_to_result,
};

use super::model::Feed;

use log::*;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/feed";

/// Create a new feed in the database
pub fn create_new_feed(db_conn: super::DbConnection, model: &Feed) -> Result<Feed, Error> {
    debug!("create_new_feed requested with model: {:?}", model);

    debug!("creating feed from model data");
    let mut feed: Feed = Feed::new(
        model.title.as_ref().unwrap().as_str(),
        model.description.as_ref().unwrap().as_str(),
        model.link.as_ref().unwrap().as_str(),
    )?;

    match feed.save(db_conn.clone(), Option::None) {
        Ok(_) => {
            debug!(
                "successfully saved feed with uuid {:?} in the database",
                feed
            );
            Result::Ok(feed)
        }
        Err(e) => {
            warn!("could not save feed: {:?} | in the database: {:?}", feed, e);
            Result::Err(create_error!(SCOPE, FeedDbError::FailedToSaveFeed))
        }
    }
}

/// Get all the feeds from the database
pub fn get_feeds(db_conn: super::DbConnection) -> Result<Vec<Feed>, Error> {
    debug!("get_feeds requested");

    match Feed::find(db_conn.clone(), Option::None, Option::None) {
        Ok(_value) => {
            debug!("extracted feeds from the database");
            Result::Ok(_value)
        }
        Err(e) => {
            warn!("the database did not return any feeds: {:?}", e);
            Result::Err(create_error!(SCOPE, FeedDbError::FailedToGetFeeds))
        }
    }
}

/// Get a feed from the database, based on its uuid
pub fn get_feed(db_conn: super::DbConnection, uuid: &Uuid) -> Result<Feed, Error> {
    debug!("get_feed requested with uuid: {}", uuid);

    let feeds: Vec<Feed> = get_feeds(db_conn)?;

    for feed in feeds {
        trace!("iterating over feed {:?}", feed);
        if feed.get_uuid().is_none() {
            trace!("feed {:?} has no uuid, skipping", feed);
            continue;
        }
        if feed.get_uuid().as_ref().unwrap() == uuid {
            debug!("found a feed: {:?}", feed);
            return Result::Ok(feed);
        }
    }

    Result::Err(create_error!(SCOPE, FeedDbError::NoFeedFound))
}

/// Get the checksum for a feed, based on its uuid
pub fn get_feed_checksum(db_conn: super::DbConnection, uuid: &Uuid) -> Result<String, Error> {
    debug!("get_feed_checksum requested with uuid: {}", uuid);

    let feed: Feed = get_feed(db_conn, uuid)?;

    option_to_result!(feed.get_checksum(), SCOPE, FeedDbError::FeedHasNoChecksum)
}

/// Update the contents of a feed, based on its uuid
pub fn update_feed(db_conn: super::DbConnection, uuid: &Uuid, model: &Feed) -> Result<Feed, Error> {
    debug!("update_feed requested with feed model: {:?}", model);

    let prev_feed = get_feed(db_conn.clone(), uuid)?;
    Result::Ok(prev_feed)
}

pub fn delete_feed(db_conn: super::DbConnection, uuid: Uuid) -> Result<Report<String>, Error> {
    debug!("delete_feed requested with uuid {}", uuid);

    let prev_feed = get_feed(db_conn.clone(), uuid)?;
    if let Err(e) = prev_feed.delete(db_conn.clone()) {
        warn!(
            "{}",
            format!("failed to delete the feed with the uuid {}: {:?}", uuid, e)
        );
        return Result::Err(create_error!(SCOPE, FeedDbError::FailedToDeleteFeed));
    }

    Result::Ok(Report::new(SCOPE.to_string(), "success".to_string()))
}
