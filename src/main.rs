use std::fs;
use std::io::Write;
use std::sync::mpsc;

use serde_json;
use serde::Deserialize;

mod logger;
mod mocap_bind;
mod vts_bind;

#[derive(Deserialize, Debug)]
pub struct Config {
    ip:String,
    port:String,
    lang:String,
    token:String
}

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

fn quit(){
    std::process::exit(0);
}

fn read_config() -> Config{
    /*for e in fs::read_dir(".\\src").unwrap(){
        println!("{:?}",e.unwrap().path())
    }*/
    let file:String = fs::read_to_string(".\\src\\config.json").expect("Config not found!"); //open and read config.json
    let cnf:Config = serde_json::from_str(&file).expect("Config file malformed!"); //parse file
    return cnf;
}

fn main() {
    //read config
    println!("reading config...");
    let mut config = read_config(); //reads json file. only thing we care about is that it has the ip address of my phone as a string
    logger::log_msg("push_enter", &config.lang, "yellow","black");
    let input = prompt("==> ");//"{} ==> ", &logger::grab_translation("push_enter", &config.lang)).as_str()); //app is multilingual. basically, press enter to start app
    if input == "quit"{
        quit();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    println!("spawning threads...");
    rt.block_on(vts_bind::vts_bind(&mut config.token));
    //let _mcbind = thread::spawn(move || mocap_bind::mocap_bind(config.ip.to_string())).join().unwrap();

    
    //todo: implement websocket
}
