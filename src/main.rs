#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod lib;

use lib::{config::CONFIG, routes, utils::rocket_config_builder};

fn main() {
    println!("debug: {}\r\n{:?}", CONFIG.debug, CONFIG.rocket);

    rocket::custom(rocket_config_builder())
        .mount("/", routes![routes::hello])
        .launch();
}
