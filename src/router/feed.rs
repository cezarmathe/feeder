#[get("/feeds/<uuid>")]
pub fn get_feed(uuid: String) {
    unimplemented!();
}

#[get("/feeds")]
pub fn get_feeds() {
    unimplemented!();
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) {
    unimplemented!();
}

#[post("/feeds")]
pub fn create_feed() {
    unimplemented!();
}

#[put("/feeds/<uuid>")]
pub fn update_feed(uuid: String) {
    unimplemented!();
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(uuid: String) {
    unimplemented!();
}
