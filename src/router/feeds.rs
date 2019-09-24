use crate::{common::error::Error, json_result, db::{model::Feed, feed}};

use std::{option::Option, result::Result, vec::Vec};

use log::{debug, warn};

use rocket_contrib::json::Json;

#[get("/feeds/<uuid>?with_items&<with_items>")]
pub fn get_feed(uuid: String, with_items: Option<bool>) -> Json<Feed> {
    unimplemented!();
}

#[get("/feeds?with_items&<with_items>")]
pub fn get_feeds(with_items: Option<bool>) -> Json<Vec<Feed>> {
    unimplemented!();
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) -> String {
    unimplemented!();
}

#[post("/feeds", format = "application/json", data = "<_feed>")]
pub fn create_feed(_feed: Json<Feed>) -> Result<Json<Feed>, Json<Error>> {
    json_result!(feed::create_new_feed(_feed.0))
}

#[put("/feeds/<uuid>", format = "application/json", data = "<feed>")]
pub fn update_feed(uuid: String, feed: Json<Feed>) -> Json<Feed> {
    unimplemented!();
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(uuid: String) {
    unimplemented!();
}
