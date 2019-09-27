use crate::{common::error::Error, json_result, db::{model::Feed, feed}};

use std::{option::Option, result::Result, str::FromStr, vec::Vec};

use log::*;

use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feeds";

#[get("/feeds/<uuid>?<with_items>")]
pub fn get_feed(uuid: String, with_items: Option<String>) -> Result<Json<Feed>, Json<Error>> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => json_result!(feed::get_feed(_value)),
        Err(e) => {
            warn!("could not decode uuid: {:?}", e);
            json_result!(Result::Err(create_error!(SCOPE, "uuid is not valid")))
        }
    }
}

#[get("/feeds?<with_items>")]
pub fn get_feeds(with_items: Option<String>) -> Result<Json<Vec<Feed>>, Json<Error>> {
    json_result!(feed::get_feeds())
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) -> String {
    unimplemented!();
}

#[post("/feeds", format = "application/json", data = "<_feed>")]
pub fn create_feed(_feed: Json<Feed>) -> Result<Json<Feed>, Json<Error>> {
    if _feed.title.is_none() {

    }
    if _feed.description.is_none() {

    }
    if _feed.link.is_none() {

    }
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
