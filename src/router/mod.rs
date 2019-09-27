mod feed_items;
mod feeds;

/// Start the router
pub fn start() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                feeds::get_feed,
                feeds::get_feeds,
                feeds::get_feed_checksum,
                feeds::create_feed,
                feeds::update_feed,
                feeds::delete_feed,
                feed_items::get_feed_item,
                feed_items::get_feed_items,
                feed_items::create_feed_item,
                feed_items::update_feed_item,
                feed_items::delete_feed_item,
            ],
        )
        .launch();
}
