use crate::common::{
    errors::{Error, FeedItemDbError, ModelError},
    report::Report,
};

use super::model::{Feed, FeedItem, ItemsVec};

use log::*;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/feed_item";

/// Get the feed items from the database
pub fn get_feed_items(db_conn: &super::DbConnection, feed: &Feed) -> Result<Vec<FeedItem>, Error> {
    if feed.items.is_none() {
        return Result::Err(create_error!(SCOPE, ModelError::NoItems));
    }

    if let ItemsVec::Full(items) = feed.items.as_ref().unwrap() {
        return Result::Ok(items.to_vec());
    }

    let items: Vec<FeedItem>;
    match FeedItem::find(db_conn.clone(), Option::None, Option::None) {
        Ok(value) => items = value,
        Err(e) => {
            warn!("database failed to return the feed items: {:?}", e);
            return Result::Err(create_error!(SCOPE, FeedItemDbError::FailedToGetItems));
        }
    };

    // for item_uuid in feed.items {}

    Result::Ok(items)
}

/// Get a feed item from the database
pub fn get_feed_item(
    db_conn: &super::DbConnection,
    feed: &Feed,
    uuid: &Uuid,
) -> Result<FeedItem, Error> {
    let items: Vec<FeedItem> = get_feed_items(db_conn, feed)?;

    for item in items {
        if let Some(item_uuid) = item.get_uuid() {
            if item_uuid == *uuid {
                return Result::Ok(item);
            }
        }
    }
    Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound))
}
