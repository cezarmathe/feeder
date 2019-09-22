mod feed;
mod feed_items;

pub fn start() {
    rocket::ignite()
        .mount("/watch",
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
               ])
        .launch();
}