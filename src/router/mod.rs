pub fn start() {
    rocket::ignite()
//        .mount("/watch",
//               routes![watch::get_watcher_info,
//               watch::create_new_watcher,
//               watch::delete_watcher])
//        .mount("/user", routes![user::create_user])
        .launch();
}