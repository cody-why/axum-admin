use std::collections::HashMap;
use std::env::args;

use serde::Deserialize;
use crate::error::Error;

pub mod db;


fn init_config() -> Config {
    let config_file = args().nth(1).unwrap_or_else(|| "config/application.yaml".to_string());
    
    #[cfg(not(debug_assertions))]
    {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("");
        // println!("current_exe: {:?}", path);
        std::env::set_current_dir(path).unwrap();
    }
    println!("current_dir: {:?}", std::env::current_dir().unwrap());

    let file = std::fs::File::open(config_file).expect("failed to open file");
    let mut config: Config = serde_yaml::from_reader(file).expect("failed to parse file");
    let errors = init_error_config().errors;
    config.errors = errors;
    config
}

fn init_error_config() -> ErrorConfig {
    let config_file =  "config/errors.yaml";
    let file = std::fs::File::open(config_file).expect("failed to open file");
    serde_yaml::from_reader(file).expect("failed to parse file")
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub addr: String,
    pub cache_type: String,
    pub jwt_secret: String,
    pub jwt_exp: usize,
    pub jwt_refresh_token: usize,
    pub white_list_api: Vec<String>,
    pub login_fail_retry: usize,
    pub login_fail_retry_wait_sec: usize,
    pub trash_recycle_days: usize,
    pub datetime_format: String,
    // pub log: LogConfig,
    pub db: DBConfig,
    pub redis: RedisConfig,
    #[serde(skip)]
    pub errors: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        init_config()
    }

    pub fn get_error(&self, code: &str) -> Error {
        let error_info = self.get_error_info(code);
        Error::Code(code.to_string(), error_info)
    }

    pub fn get_error_arg(&self, code: &str, arg: String) -> Error {
        let error_info = self.get_error_info(code);
        Error::Code(code.to_string(), error_info.replace("{}", &arg))
    }

    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => "未知错误".to_string(),
            Some(v) =>v.to_string() ,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub file_path: String,
    pub file_name: String,
    pub to_file: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: "logs".to_string(),
            file_name: "app.log".to_string(),
            to_file: false
        }
    }
}

#[derive(Debug,Default, Deserialize)]
pub struct DBConfig{
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u32,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedisConfig{
    pub url: String,
}

#[derive(Debug,Default, Deserialize)]
pub struct ErrorConfig{
    pub errors: HashMap<String,String>,
}

