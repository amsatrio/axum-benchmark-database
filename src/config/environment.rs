use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;



// get data from environment manual
pub fn get_server_url() -> Result<String, String> {
    dotenv::dotenv().ok();
    let server_host = env::var("SERVER_HOST").expect("SERVER_HOST is not set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT is not set");
    Ok(format!("{}:{}", server_host, server_port))
}

// get data from environment using dotenv, then serialize to Config model
pub fn get_config() -> Environment {
    dotenv().ok();

    match envy::from_env::<Environment>() {
        Ok(config) => config,
        Err(error) => panic!("Environment Error: {:#?}", error),
    }
}

// get data from json
pub fn get_config_from_json() -> Environment {
    let config_file: &'static str = "env.json";
    let config = Environment::from_file(config_file);
    config
}

// save Config model to heap (avoid repeated serialization)
lazy_static! {
    pub static ref CONFIG: Environment = get_config();
}




use std::fs;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Environment {

    pub database_username: String,
    pub database_password: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_dbname: String,
    pub database_max_pool: u32,
    pub database_min_pool: u32,

    pub server_host: String,
    pub server_port: u16,
    pub server_thread: usize,

}

impl Environment {
    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    pub fn get_server_url(&self) -> String {
        format!("{0}:{1}", self.server_host, self.server_port)
    }

    pub fn get_database_url(&self) -> String {
        return format!(
            "postgresql://{0}:{1}@{2}:{3}/{4}",
            self.database_username,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_dbname
        );
    }
}
