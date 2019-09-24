use crate::{
    db::model,
    router::RouterError
};

use std::{
    option::Option,
    result::Result,
    vec::Vec
};

use log::{debug, warn};
use mongodb::{
    db::ThreadedDatabase,
    ThreadedClient
};
use wither::prelude::Model;

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
    data = "<_feed>"
)]
pub fn create_feed(_feed: Json<model::Feed>) -> Result<Json<model::Feed>, Json<RouterError>> {
    let client = &crate::DB_CLIENT;
    let db = client.db("feeder");

    let mut feed: model::Feed;
    match model::Feed::new(
        _feed.0.title.as_str(),
        _feed.0.description.as_str(),
        _feed.0.link.as_str()) {
        Some(_value) => {
            feed = _value;
        },
        None => {
            warn!("failed to create new feed");
            return Result::Err(Json(RouterError::new_from_str("unexpected error when creating a feed")));
        }
    }

    debug!("saving the feed {:?} in the database", feed);
    match feed.save(db.clone(), Option::None) {
        Ok(_) => {},
        Err(e) => {
            warn!("failed to save the new feed in the database: {:?}", e);
            return Result::Err(Json(RouterError::new_from_str("failed to save the feed in the database")));
        }
    }

    return Result::Ok(Json(feed));
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
