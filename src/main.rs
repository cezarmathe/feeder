#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate mongodb;

mod db;
mod router;

use std::env;

use log::{debug, info, error};
use mongodb::Client;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;

lazy_static! {
    static ref LOG_FILE: &'static str = {
        match env::var("LOG_FILE").unwrap_or(String::from("STDOUT")).as_str() {
                "FILE" => "feeder.log",
                "STDERR" => "/dev/stderr",
                "DEVNULL" => "/dev/null",
                _ => "/dev/stdout"
        }
    };
    static ref LOG_LEVEL_FILTER: log::LevelFilter = {
        match env::var("LOG_LEVEL").unwrap_or(String::from("INFO")).as_str() {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            "OFF" => log::LevelFilter::Off,
            _ => log::LevelFilter::Info
        }
    };
    static ref _LOG_BYTE: u8 = {
        simple_logging::log_to_file(*LOG_FILE, *LOG_LEVEL_FILTER).unwrap();
        0
    };
    static ref DB_HOST: String = {
        debug!("retrieving database host");
        match env::var("DB_HOST") {
            Ok(_value) => {
                debug!("database host is: {}", _value);
                _value
            },
            Err(e) => {
                error!("Failed to get the database host from the environment variable DB_HOST: {:?}", e);
                panic!();
            }
        }
    };
    static ref DB_PORT: u16 = {
        debug!("retrieving database port");
        let _port: String = env::var("DB_PORT").unwrap_or(String::from("27017"));
        match _port.parse::<u16>() {
            Ok(_value) => {
                debug!("database port is: {}", _value);
                _value
            },
            Err(e) => {
                error!("Failed to get the database port from the environment variable DB_PORT: {:?}", e);
                panic!();
            }
        }
    };
    static ref DB_CLIENT: Client = {
        match Client::connect(&*DB_HOST, *DB_PORT) {
            Ok(_db_client) => {
                info!("Initialized database with hostname {} and port {}", *DB_HOST, *DB_PORT);
                _db_client
            },
            Err(e) => {
                error!("Failed to initialize database: {:?}", e);
                panic!();
            }
        }
    };
}

fn main() {
    *_LOG_BYTE;
    debug!("started main");

    DB_CLIENT.db("feeder").collection("feeds");
    DB_CLIENT.db("feeder").collection("feed_items");

    debug!("starting router");
    router::start();
}
