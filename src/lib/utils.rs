use crate::lib::config::CONFIG;

pub fn rocket_config_builder() -> rocket::Config {
    rocket::Config::build(match CONFIG.rocket.environment.as_str() {
        "production" => rocket::config::Environment::Production,
        "staging" => rocket::config::Environment::Staging,
        _ => rocket::config::Environment::Development,
    })
    .address(CONFIG.rocket.address.as_str())
    .port(CONFIG.rocket.port)
    .workers(CONFIG.rocket.workers)
    .unwrap()
}
