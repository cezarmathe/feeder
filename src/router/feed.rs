use crate::db::model;

use std::option::Option;
use std::vec::Vec;

use log::{debug, warn};
use mongodb::bson;
use mongodb::Client;
use mongodb::db::ThreadedDatabase;
use mongodb::ThreadedClient;
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
    let client = crate::DB_CLIENT.__private_field;
    let feed_collection = client.db("feeder").collection("feeds");
    debug!("retrieved feed collection from the database");

    let json_string: String;

    match feed.to_json() {
        Some(_value) => json_string = _value,
        None => {
            warn!("failed to complete requested due to json failure");
            return feed;
        }
    }

    match feed_collection.insert_one(bson!(json_string)) {
        Ok(_result) => {
            debug!("result: {:?}", _result);
        },
        Err(e) => {
            warn!("failed to insert feed: {:?}", e)
        }
    }
    return feed;
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
