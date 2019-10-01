use crate::db::model;

use std::vec::Vec;

use rocket_contrib::json::Json;

#[get("/feeds/<_feed_uuid>/items/<_item_uuid>")]
pub fn get_feed_item(_feed_uuid: String, _item_uuid: String) -> Json<model::FeedItem> {
    unimplemented!();
}

#[get("/feeds/<_feed_uuid>/items")]
pub fn get_feed_items(_feed_uuid: String) -> Json<Vec<model::FeedItem>> {
    unimplemented!();
}

#[post(
    "/feeds/<_feed_uuid>/items",
    format = "application/json",
    data = "<_feed_item>"
)]
pub fn create_feed_item(
    _feed_uuid: String,
    _feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
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
    _feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
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
    _feed_item: Json<model::FeedItem>,
) -> Json<model::FeedItem> {
    unimplemented!();
}
