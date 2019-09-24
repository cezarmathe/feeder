mod feed;
mod feed_items;

use std::time::{SystemTime, UNIX_EPOCH};

/// Start the router
pub fn start() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                feed::get_feed,
                feed::get_feeds,
                feed::get_feed_checksum,
                feed::create_feed,
                feed::update_feed,
                feed::delete_feed,
                feed_items::get_feed_item,
                feed_items::get_feed_items,
                feed_items::create_feed_item,
                feed_items::update_feed_item,
                feed_items::delete_feed_item,
            ],
        )
        .launch();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Error reported by a router endpoint
pub struct RouterError {
    timestamp: u64,
    message: String
}

impl RouterError {
    /// Create a new struct using a &str
    pub fn new_from_str(message: &str) -> RouterError {
        Self::new(String::from(message))
    }

    /// Create a new struct using a String
    pub fn new(message: String) -> RouterError {
        let time: u64;
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(_value) => time = _value.as_secs(),
            Err(e) => {
                // extremely bad if happens
                panic!(e);
            }
        }
        RouterError {
            timestamp: time,
            message
        }
    }
}
