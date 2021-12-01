extern crate serde;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip:String,
    pub port:String,
    pub lang:String,
    pub token:String
}

impl Config{
    pub fn set_token(&mut self, tk:String){
        self.token = tk;
    }
}

#[derive(Clone)]
pub struct SharedConfig {
    pub shared:Arc<Mutex<Config>>,
}

impl SharedConfig {
    pub fn set_token(&self, tk:String) {
        let mut lock = self.shared.lock().unwrap();
        lock.token = tk;
    }
    pub fn get_token(&self) -> String {
        let lock = self.shared.lock().unwrap();
        return lock.token.clone();
    }
}