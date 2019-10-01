use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
};

pub struct RequestCounter {
    count: AtomicUsize,
}

impl RequestCounter {
    pub fn new() -> RequestCounter {
        RequestCounter {
            count: AtomicUsize::new(0),
        }
    }
}

impl Fairing for RequestCounter {
    fn info(&self) -> Info {
        Info {
            name: "Request Counter",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        self.count.fetch_add(1, Ordering::Relaxed);
        //        println!("Number of requests received: {}", self.count.load(Ordering::Relaxed));
    }
}
