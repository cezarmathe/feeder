#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use(Model)]
extern crate wither_derive;

#[macro_use]
mod common;
#[macro_use]
mod db;
mod router;

use std::env;

use log::*;

lazy_static! {
    static ref _LOG_FILE: &'static str = {
        match env::var("LOG_FILE")
            .unwrap_or(String::from("STDOUT"))
            .as_str()
        {
            "FILE" => "feeder.log",
            "STDERR" => "/dev/stderr",
            "DEVNULL" => "/dev/null",
            _ => "/dev/stdout",
        }
    };
    static ref _LOG_LEVEL_FILTER: log::LevelFilter = {
        match env::var("LOG_LEVEL")
            .unwrap_or(String::from("INFO"))
            .as_str()
        {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            "OFF" => log::LevelFilter::Off,
            _ => log::LevelFilter::Info,
        }
    };
    static ref _LOG: u8 = {
        simple_logging::log_to_file(*_LOG_FILE, *_LOG_LEVEL_FILTER).unwrap();
        0
    };
}

fn main() {
    #![allow(warnings)]
    *_LOG;
    debug!("started main");

    debug!("starting router");
    router::start();
}
