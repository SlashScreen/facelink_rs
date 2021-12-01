//main.rs
//main app
//include std
use std::fs;
use std::io::Write;
use std::sync::mpsc;
use std::{thread};
use std::sync::{Arc, Mutex};
//include vendor
use tokio::runtime::Runtime;
use serde_json;
use fsconfig;
use fllog;

//include modules
mod mocap_bind;
mod vts_bind;

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

fn read_config() -> fsconfig::Config{
    let file:String = fs::read_to_string(".\\src\\config.json").expect("Config not found!"); //open and read config.json
    let cnf:fsconfig::Config = serde_json::from_str(&file).expect("Config file malformed!"); //parse file
    return cnf;
}

fn main() {
    //read config
    println!("reading config...");
    let config = Arc::new(Mutex::new(read_config())) ; //reads json file. only thing we care about is that it has the ip address of my phone as a string


    fllog::log_msg("push_enter", &config.lock().unwrap().lang, "yellow","black");
    let input = prompt("==> "); //app is multilingual. basically, press enter to start app

    if input == "quit"{
        quit();
    }

    println!("spawning threads...");
    let runtime = Runtime::new().unwrap();
    let (vtx_tx, vtx_rx) = mpsc::channel();
    let (ifm_tx, ifm_rx) = mpsc::channel::<String>();

    println!("Spawning Mocap thread...");
    let m_cnfg = config.clone();
    thread::spawn(move || {
        let ip = m_cnfg.lock().unwrap().ip.clone();
        mocap_bind::mocap_bind(ifm_tx,ip).expect("could not bind");
    });
    //let _ = mc.join().expect("Error with IFacialMocap Bind");
    println!("Spawning VTS thread...");
    let c_cnfg = fsconfig::SharedConfig{shared:config};
    runtime.spawn(async move {
        vts_bind::vts_bind(vtx_rx,&c_cnfg).await
    });


    loop{
        let d = ifm_rx.recv().expect("error receiving"); //get from ifacialmocap, block until it gets
        let _ = vtx_tx.send(d); //send data to vts
    }
}
