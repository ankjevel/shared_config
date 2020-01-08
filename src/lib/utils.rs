use crate::lib::config::CONFIG;
use rocket::{
    config::{Environment::*, LoggingLevel::*},
    Config,
};

pub fn rocket_config_builder() -> rocket::Config {
    let mut conf = Config::build(match CONFIG.rocket.environment.as_str() {
        "production" => Production,
        "staging" => Staging,
        _ => Development,
    })
    .address(CONFIG.rocket.address.as_str())
    .port(CONFIG.rocket.port)
    .workers(CONFIG.rocket.workers)
    .log_level(match CONFIG.rocket.log_level.as_str() {
        "critical" => Critical,
        "debug" => Debug,
        "off" => Off,
        _ => Normal,
    });

    if let Some(keep_alive) = CONFIG.rocket.keep_alive {
        conf = conf.keep_alive(keep_alive);
    }

    if let Some(secret_key) = &CONFIG.rocket.secret_key {
        conf = conf.secret_key(secret_key);
    }

    conf.unwrap()
}
