pub mod rocket;

use self::rocket::Rocket;
use config::{Config as C, Environment, File};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub debug: bool,
    pub rocket: Rocket,
}

fn set_defaults(config: &mut C) {
    config.set_default("debug", false).unwrap();
    Rocket::default_config().merge_with_config(config, "rocket.");
}

fn merge_with_config_file(config: &mut C) {
    let config_path = Path::new("config.json");
    if config_path.exists() {
        config.merge(File::from(config_path)).unwrap();
    }
}

fn merge_with_env(config: &mut C) {
    let env = Environment::new();
    let env = env.separator("__");
    let env = env.ignore_empty(true);
    config.merge(env).unwrap();
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = C::new();

        set_defaults(&mut config);
        merge_with_env(&mut config);
        merge_with_config_file(&mut config);

        config.try_into::<Config>().unwrap()
    };
}
