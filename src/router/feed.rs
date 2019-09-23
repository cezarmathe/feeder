use crate::db::model;

use std::vec::Vec;

use rocket_contrib::json::Json;

#[get("/feeds/<uuid>?with_items&<with_items>")]
pub fn get_feed(uuid: String, with_items: Option<bool>) -> Json<model::Feed> {
    unimplemented!();
}

#[get("/feeds?with_items&<with_items>")]
pub fn get_feeds(with_items: Option<bool>) -> Json<Vec<model::Feed>> {
    unimplemented!();
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) -> String {
    unimplemented!();
}

#[post(
    "/feeds",
    format = "application/json",
    data = "<feed>"
)]
pub fn create_feed(feed: Json<model::Feed>) -> Json<model::Feed> {
    unimplemented!();
}

#[put(
    "/feeds/<uuid>",
    format = "application/json",
    data = "<feed>"
)]
pub fn update_feed(uuid: String, feed: Json<model::Feed>) -> Json<model::Feed> {
    unimplemented!();
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(uuid: String) {
    unimplemented!();
}
