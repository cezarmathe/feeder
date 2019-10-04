use crate::{
    common::{check_uuid, report::Report, JsonResult},
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
    match feed::get_feed(db_conn.clone(), good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }
    if feed.items.is_none() {
        json_result!(Result::Err(create_error!(SCOPE, "feed has no items")))
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
    let err_msg = format!(
        "no feed item with uuid {} found for the feed with uuid {}",
        good_item_uuid, good_item_uuid
    );
    json_result!(Result::Err(create_error!(SCOPE, err_msg)))
}

#[get("/feeds/<_feed_uuid>/items")]
pub fn get_feed_items(_feed_uuid: String) -> Json<Vec<FeedItem>> {
    unimplemented!();
}

#[post(
    "/feeds/<_feed_uuid>/items",
    format = "application/json",
    data = "<_feed_item>"
)]
pub fn create_feed_item(_feed_uuid: String, _feed_item: Json<FeedItem>) -> Json<FeedItem> {
    unimplemented!();
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
