use config::Config as C;
use rocket::config::{Config, Environment, LoggingLevel};

#[derive(Debug, Deserialize, Serialize)]
pub enum EnvironmentDef {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LoggingLevelDef {
    Critical,
    Normal,
    Debug,
    Off,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rocket {
    pub environment: String,
    pub address: String,
    pub port: u16,
    pub workers: u16,
    pub keep_alive: Option<u32>,
    pub log_level: String,
}

impl Rocket {
    pub fn default_config() -> Rocket {
        let default = Config::new(Environment::Development);

        let keep_alive = if default.keep_alive.is_some() {
            Some(default.keep_alive.unwrap() as u32)
        } else {
            None
        };

        Rocket {
            environment: "development".to_string(),
            address: default.address,
            port: default.port as u16,
            workers: default.workers as u16,
            keep_alive,
            log_level: match default.log_level {
                LoggingLevel::Critical => "critical",
                LoggingLevel::Normal => "normal",
                LoggingLevel::Debug => "debug",
                LoggingLevel::Off => "off",
            }
            .to_string(),
        }
    }

    pub fn merge_with_config(&self, config: &mut C, prefix_str: &str) {
        let prefix = |key: &str| -> String { prefix_str.to_string() + key };

        config
            .set_default(&prefix("environment"), self.environment.to_string())
            .unwrap();
        config
            .set_default(&prefix("address"), self.address.to_string())
            .unwrap();
        config
            .set_default(&prefix("port"), self.port as i64)
            .unwrap();
        config
            .set_default(&prefix("workers"), self.workers as i64)
            .unwrap();
        config
            .set_default(&prefix("log_level"), self.log_level.to_string())
            .unwrap();
        if let Some(keep_alive) = self.keep_alive {
            config
                .set_default(&prefix("keep_alive"), keep_alive as i64)
                .unwrap();
        }
    }
}
