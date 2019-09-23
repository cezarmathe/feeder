use crate::db::model;

use std::vec::Vec;

use rocket_contrib::json::Json;

#[get("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn get_feed_item(feed_uuid: String, item_uuid: String) -> Json<model::FeedItem> {
    unimplemented!();
}

#[get("/feeds/<feed_uuid>/items")]
pub fn get_feed_items(feed_uuid: String) -> Json<Vec<model::FeedItem>> {
    unimplemented!();
}

#[post(
    "/feeds/<feed_uuid>/items",
    format = "application/json",
    data = "<feed_item>"
)]
pub fn create_feed_item(
    feed_uuid: String,
    feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
    unimplemented!();
}

#[put(
    "/feeds/<feed_uuid>/items/<item_uuid>",
    format = "application/json",
    data = "<feed_item>"
)]
pub fn update_feed_item(
    feed_uuid: String,
    item_uuid: String,
    feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
    unimplemented!();
}

#[delete(
    "/feeds/<feed_uuid>/items/<item_uuid>",
    format = "application/json",
    data = "<feed_item>"
)]
pub fn delete_feed_item(
    feed_uuid: String,
    item_uuid: String,
    feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
    unimplemented!();
}
