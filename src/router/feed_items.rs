use super::check_uuid;

use crate::{
    common::{errors::FeedItemsRouterError, report::Report, JsonResult},
    db::{
        model::{Feed, FeedItem},
        DbConnection, FeedItemWrapper, FeedWrapper,
    },
    json_result,
};

use std::{option::Option, result::Result, vec::Vec};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feed_items";

#[get("/feeds/<feed_uuid>/items")]
pub fn get_all_feed_items(db_conn: DbConnection, feed_uuid: String) -> JsonResult<Vec<FeedItem>> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its feed items uuids
    let feed: Feed;
    match (&*db_conn).clone().get_feed(good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    json_result!((&*db_conn).clone().get_feed_items(feed, Option::None))
}

#[get("/feeds/<feed_uuid>/items/<item_uuids>")]
pub fn get_specific_feed_items(
    db_conn: DbConnection,
    feed_uuid: String,
    item_uuids: Option<String>,
) -> JsonResult<Vec<FeedItem>> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its feed items uuids
    let feed: Feed;
    match (&*db_conn).clone().get_feed(good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    if item_uuids.is_none() {
        info!("no item uuids found, fetching all items for this feed");
        json_result!((&*db_conn).clone().get_feed_items(feed, Option::None))
    }

    let mut good_item_uuids: Vec<Uuid> = Vec::new();
    for item_uuid in item_uuids.unwrap().split(",") {
        match check_uuid(item_uuid.to_string(), SCOPE) {
            Ok(value) => good_item_uuids.push(value),
            Err(e) => {
                json_result!(Result::Err(e));
            }
        }
    }

    json_result!((&*db_conn)
        .clone()
        .get_feed_items(feed, Option::Some(good_item_uuids)))
}

#[post(
    "/feeds/<feed_uuid>/items",
    format = "application/json",
    data = "<model>"
)]
pub fn create_feed_item(
    db_conn: DbConnection,
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

    // Check if the feed exists and get its feed items uuids
    let feed: Feed;
    match (&*db_conn).clone().get_feed(good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    json_result!((&*db_conn).clone().create_feed_item(feed, model.0))
}

#[put(
    "/feeds/<feed_uuid>/items/<item_uuid>",
    format = "application/json",
    data = "<feed_item>"
)]
pub fn update_feed_item(
    db_conn: DbConnection,
    feed_uuid: String,
    item_uuid: String,
    feed_item: Json<FeedItem>,
) -> JsonResult<FeedItem> {
    // Check if the uuids are valid
    let good_feed_uuid: Uuid;
    match check_uuid(feed_uuid, SCOPE) {
        Ok(value) => good_feed_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }
    // Check if the uuids are valid
    let good_item_uuid: Uuid;
    match check_uuid(item_uuid, SCOPE) {
        Ok(value) => good_item_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed exists and get its feed items uuids
    let feed: Feed;
    match (&*db_conn).clone().get_feed(good_feed_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    json_result!((&*db_conn)
        .clone()
        .update_feed_item(feed, good_item_uuid, feed_item.0))
}

#[delete("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn delete_feed_item(
    db_conn: DbConnection,
    feed_uuid: String,
    item_uuid: String,
) -> JsonResult<Report<String>> {
    unimplemented!();
}
