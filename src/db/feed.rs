use crate::{common::error::Error, create_error, option_to_result};

use super::model::Feed;

use std::{option::Option, vec::Vec};

use log::*;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/feed";

pub fn create_new_feed(db_conn: super::FeederDbConn, model: Feed) -> Result<Feed, Error> {
    debug!("create_new_feed requested with model: {:?}", model);

    debug!("creating feed from model data");
    let mut feed: Feed = Feed::new(
        model.title.unwrap().as_str(),
        model.description.unwrap().as_str(),
        model.link.unwrap().as_str(),
    )?;

    match feed.save(db_conn.0.clone(), Option::None) {
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

pub fn get_feeds(db_conn: super::FeederDbConn) -> Result<Vec<Feed>, Error> {
    debug!("get_feeds requested");

    match Feed::find(db_conn.0.clone().clone(), Option::None, Option::None) {
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

pub fn get_feed(db_conn: super::FeederDbConn, uuid: Uuid) -> Result<Feed, Error> {
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

pub fn get_feed_checksum(db_conn: super::FeederDbConn, uuid: Uuid) -> Result<String, Error> {
    debug!("get_feed_checksum requested with uuid: {}", uuid);

    let feed: Feed = get_feed(db_conn, uuid)?;

    let err_msg: String = format!("feed with uuid {} does not have a checksum", uuid);
    option_to_result!(feed.get_checksum(), SCOPE, err_msg)
}
