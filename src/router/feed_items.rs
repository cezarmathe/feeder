use super::check_uuid;

use crate::{
    common::{errors::FeedItemsRouterError, report::Report, JsonResult},
    db::{
        feed,
        model::{Feed, FeedItem, ItemsVec},
        FeederDbConn,
    },
    json_result,
};

use std::{option::Option, result::Result, vec::Vec};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feed_items";

#[get("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn get_feed_item(
    db_conn: FeederDbConn,
    feed_uuid: String,
    item_uuid: String,
) -> JsonResult<FeedItem> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }
    let good_item_uuid: Uuid;
    match check_uuid(item_uuid, SCOPE) {
        Ok(value) => good_item_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its feed items uuids
    let feed: Feed;
    match feed::get_feed(db_conn.clone(), &good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }
    if feed.items.is_none() {
        json_result!(Result::Err(create_error!(
            SCOPE,
            FeedItemsRouterError::FeedHasNoItems
        )))
    }
    let mut feed_item_uuid_vec: Vec<Uuid> = Vec::new();
    match feed.items.unwrap() {
        ItemsVec::Uuid(uuid_vec) => {
            for item_uuid in uuid_vec {
                feed_item_uuid_vec.push(item_uuid);
            }
        }
        ItemsVec::Full(item_vec) => {
            for item in item_vec {
                if let Some(value) = item.get_uuid() {
                    feed_item_uuid_vec.push(value);
                }
            }
        }
    }

    // Check if the feed has a feed item with this uuid
    for item_uuid in feed_item_uuid_vec {
        if item_uuid == good_item_uuid {
            // TODO: get the actual feed item from the database
            json_result!(FeedItem::new("a", "b", "c"))
        }
    }

    // Otherwise, return an error
    json_result!(Result::Err(create_error!(
        SCOPE,
        FeedItemsRouterError::NoFeedItemInFeed
    )))
}

#[get("/feeds/<feed_uuid>/items")]
pub fn get_feed_items(db_conn: FeederDbConn, feed_uuid: String) -> JsonResult<Vec<FeedItem>> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its items
    let mut feed: Feed;
    match feed::get_feed(db_conn.clone(), &good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }
    if feed.items.is_none() {
        json_result!(Result::Err(create_error!(
            SCOPE,
            FeedItemsRouterError::FeedHasNoItems
        )))
    }

    // If the feed does not contain the full items, retrieve all the items
    if let ItemsVec::Uuid(_) = feed.items.clone().unwrap() {
        feed.with_items(db_conn.clone());
    }

    if let ItemsVec::Full(feed_items) = feed.items.unwrap() {
        json_result!(Result::Ok(feed_items))
    } else {
        json_result!(Result::Err(create_error!(
            SCOPE,
            FeedItemsRouterError::FeedHasNoItems
        )))
    }
}

#[post(
    "/feeds/<feed_uuid>/items",
    format = "application/json",
    data = "<model>"
)]
pub fn create_feed_item(
    db_conn: FeederDbConn,
    feed_uuid: String,
    model: Json<FeedItem>,
) -> JsonResult<FeedItem> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its items
    let mut parent_feed: Feed;
    match feed::get_feed(db_conn.clone(), &good_feed_uuid) {
        Ok(value) => parent_feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Retrieve the uuid list of items from the feed
    let mut feed_item_uuid_vec: Vec<Uuid>;
    if let Some(items_vec) = parent_feed.items {
        match items_vec {
            ItemsVec::Uuid(vec) => {
                feed_item_uuid_vec = vec;
            }
            ItemsVec::Full(vec) => {
                feed_item_uuid_vec = Vec::new();
                for item in vec {
                    if let Some(value) = item.get_uuid() {
                        feed_item_uuid_vec.push(value);
                    }
                }
            }
        }
    } else {
        feed_item_uuid_vec = Vec::new();
    }

    // Save the feed item in the database
    let feed_item: FeedItem;
    if let Ok(value) = FeedItem::new(
        // TODO: actually add the feed item in the database
        model.title.as_str(),
        model.link.as_str(),
        model.description.as_str(),
    ) {
        feed_item = value;
    } else {
        json_result!(Result::Err(create_error!(
            SCOPE,
            FeedItemsRouterError::CouldNotCreateFeedItem
        )))
    }

    // Add the feed item in the feed
    feed_item_uuid_vec.push(feed_item.get_uuid().unwrap()); // FIXME: UNSAFE!!!
    parent_feed.items = Option::Some(ItemsVec::Uuid(feed_item_uuid_vec));

    // Update the feed's checksum
    if let Some(value) = parent_feed.compute_checksum(Option::Some(db_conn.clone())) {
        json_result!(Result::Err(value));
    }

    // Update the feed in the database
    if let Err(e) = feed::update_feed(db_conn.clone(), &good_feed_uuid, parent_feed) {
        json_result!(Result::Err(e))
    }

    json_result!(Result::Ok(feed_item))
}

#[put(
    "/feeds/<_feed_uuid>/items/<_item_uuid>",
    format = "application/json",
    data = "<_feed_item>"
)]
pub fn update_feed_item(
    _feed_uuid: String,
    _item_uuid: String,
    _feed_item: Json<FeedItem>,
) -> Json<FeedItem> {
    unimplemented!();
}

#[delete(
    "/feeds/<_feed_uuid>/items/<_item_uuid>",
    format = "application/json",
    data = "<_feed_item>"
)]
pub fn delete_feed_item(
    _feed_uuid: String,
    _item_uuid: String,
    _feed_item: Json<FeedItem>,
) -> Json<FeedItem> {
    unimplemented!();
}
