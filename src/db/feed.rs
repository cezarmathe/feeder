use crate::{
    common::{error::Error, report::Report},
    create_error, option_to_result,
};

use super::model::Feed;

use std::{option::Option, vec::Vec};

use log::*;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/feed";

/// Create a new feed in the database
pub fn create_new_feed(db_conn: super::DbConn, model: Feed) -> Result<Feed, Error> {
    debug!("create_new_feed requested with model: {:?}", model);

    debug!("creating feed from model data");
    let mut feed: Feed = Feed::new(
        model.title.unwrap().as_str(),
        model.description.unwrap().as_str(),
        model.link.unwrap().as_str(),
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
            Result::Err(create_error!(
                SCOPE,
                "error occurred when saving the feed in the database"
            ))
        }
    }
}

/// Get all the feeds from the database
pub fn get_feeds(db_conn: super::DbConn) -> Result<Vec<Feed>, Error> {
    debug!("get_feeds requested");

    match Feed::find(db_conn.clone(), Option::None, Option::None) {
        Ok(_value) => {
            debug!("extracted feeds from the database");
            Result::Ok(_value)
        }
        Err(e) => {
            warn!("the database did not return any feeds: {:?}", e);
            Result::Err(create_error!(
                SCOPE,
                "the database could not return any feed"
            ))
        }
    }
}

/// Get a feed from the database, based on its uuid
pub fn get_feed(db_conn: super::DbConn, uuid: Uuid) -> Result<Feed, Error> {
    debug!("get_feed requested with uuid: {}", uuid);

    let feeds: Vec<Feed> = get_feeds(db_conn)?;

    for feed in feeds {
        trace!("iterating over feed {:?}", feed);
        if feed.get_uuid().is_none() {
            trace!("feed {:?} has no uuid, skipping", feed);
            continue;
        }
        if feed.get_uuid().unwrap() == uuid {
            debug!("found a feed: {:?}", feed);
            return Result::Ok(feed);
        }
    }

    Result::Err(create_error!(
        SCOPE,
        format!("did not find any feeds with the UUID {}", uuid)
    ))
}

/// Get the checksum for a feed, based on its uuid
pub fn get_feed_checksum(db_conn: super::DbConn, uuid: Uuid) -> Result<String, Error> {
    debug!("get_feed_checksum requested with uuid: {}", uuid);

    let feed: Feed = get_feed(db_conn, uuid)?;

    let err_msg: String = format!("feed with uuid {} does not have a checksum", uuid);
    option_to_result!(feed.get_checksum(), SCOPE, err_msg)
}

/// Update the contents of a feed, based on its uuid
pub fn update_feed(db_conn: super::DbConn, uuid: Uuid, model: Feed) -> Result<Feed, Error> {
    debug!("update_feed requested with feed model: {:?}", model);

    let prev_feed = get_feed(db_conn.clone(), uuid)?;
    Result::Ok(prev_feed)
}

pub fn delete_feed(db_conn: super::DbConn, uuid: Uuid) -> Result<Report<String>, Error> {
    debug!("delete_feed requested with uuid {}", uuid);

    let prev_feed = get_feed(db_conn.clone(), uuid)?;
    if prev_feed.delete(db_conn.clone()).is_err() {
        // FIXME: consider logging the error received from the db
        return Result::Err(create_error!(SCOPE, "failed to delete the feed"));
    }

    Result::Ok(Report::new(SCOPE.to_string(), "success".to_string()))
}
