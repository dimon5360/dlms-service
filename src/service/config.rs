
use log::debug;

use serde::Deserialize;
use std::fs;
use dotenv::dotenv;
use regex::Regex;

use crate::service::endpoint;

const DEFAULT_CONFIG_PATH: &str = "./config.json";
const CONFIG_FILE_EXT_REGEXP: &str = r"^.*\.json";

pub trait ConfigInterface {
    fn init(&mut self);
    fn get_config(&self) -> &Vec<endpoint::Endpoint>;
}

#[derive(Clone)]
pub struct Config {
    _conf_path: String,
    _endpoints: Vec<endpoint::Endpoint>,
}

#[derive(Deserialize)]
struct ServiceConfig {
    ports: Vec<String>,
}

fn process_config(_path: &str) -> Option<Vec<endpoint::Endpoint>> {

    debug!("process config running ...");
    let mut endpoints = Vec::<endpoint::Endpoint>::new();

    let re = Regex::new(CONFIG_FILE_EXT_REGEXP).unwrap();
    if !re.is_match(_path) {
        panic!("config file must have *.json extension");
    }

    let contents = fs::read_to_string(_path)
        .expect("Should have been able to read the file");
    
    let c: ServiceConfig = serde_json::from_str(&contents).unwrap();
    for p in c.ports {
        debug!("endpoint {}", p.as_str());
        endpoints.push(endpoint::new(p));
    }
    return Some(endpoints)
}

pub fn version() {
    dotenv().ok();

    let version = format!(
        "Application version v.{}.{}.{}.{}",
        std::env::var("SERVICE_MAJOR_VERSION").unwrap(),
        std::env::var("SERVICE_MINOR_VERSION").unwrap(),
        std::env::var("SERVICE_PATCH_VERSION").unwrap(),
        std::env::var("SERVICE_BUILD_VERSION").unwrap()
    );

    println !("{}", version);
}

impl ConfigInterface for Config {

    fn init(&mut self) {

        debug!("config init");
            debug!("config path: {}", self._conf_path);

        match process_config(&self._conf_path) {
            Some(config) => {
                for p in config {
                    self._endpoints.push(p);
                }
            }
            None => (),
        }
    }
    
    fn get_config(&self) -> &Vec<endpoint::Endpoint> {
        return &self._endpoints;
    }
}

pub fn new() -> Box<dyn ConfigInterface> {
    return Box::new(Config {
        _endpoints: Vec::new(),
        _conf_path: String::from(DEFAULT_CONFIG_PATH),
    });
}

