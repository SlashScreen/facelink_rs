extern crate serde;
use serde::{Deserialize,Serialize};
use serde_json;
use std::sync::{Arc, Mutex};
use std::fs;

#[derive(Deserialize,Serialize, Debug)]
pub struct Config {
    pub ip:String,
    pub port:String,
    pub lang:String,
    pub token:String
}

impl Config{
    pub fn set_lang(&mut self, lang:String){
        self.lang = lang;
        self.write_self();
    }
    pub fn write_self(&self){
        fs::write(".\\src\\config.json",serde_json::to_string_pretty(&self).unwrap()).expect("Error writing to config file");
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
        lock.write_self();
    }
    pub fn get_token(&self) -> String {
        println!("getting token...");
        let lock = self.shared.lock().expect("error locking config"); //stops here
        println!("cloning token...");
        return lock.token.clone();
    }
}
