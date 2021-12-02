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
use regex::Regex;

#[macro_use]
extern crate self_update;

//include modules
mod mocap_bind;
mod vts_bind;

fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Slashscreen")
        .repo_name("facelink_rs")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
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

fn read_config() -> fsconfig::Config{
    let file:String = fs::read_to_string(".\\src\\config.json").expect("Config not found!"); //open and read config.json
    let cnf:fsconfig::Config = serde_json::from_str(&file).expect("Config file malformed!"); //parse file
    return cnf;
}

fn main() {
    //update
    println!("Updating FacelinkRS...");
    if let Err(e) = update() {
        println!("[ERROR] {}", e);
    }
    //read config
    println!("reading config...");
    let config = Arc::new(Mutex::new(read_config())) ; //reads json file. only thing we care about is that it has the ip address of my phone as a string
    let mut first_time_setup = false;
    //SELECT LANGUAGE
    if config.lock().unwrap().lang.as_str() == ""{
        first_time_setup = true;
        let mut valid = false;
        while !valid{
            println!("Please select a language. Type 'en' to use English.");
            println!("言語を選択してください。日本語を使う場合は「jp」と入力してください。");
            println!("Bitte wählen eine Sprache. Deutsch zu benutzen, 'de' tippen ab.");
            let input = prompt("==> ");
            if input == "en".to_owned() || input == "jp".to_owned() || input == "de".to_owned(){ //really dumb way of doing this but it works
                config.lock().unwrap().set_lang(input); //set language
                valid = true;
            }else{
                println!("Invalid input. Please type 'en', 'jp', or 'de'.");
                println!("入力が無効です。「en」、「jp」、「de」を入力してください。");
                println!("Ungültige Eingabe. Bitte geben Sie 'en', 'jp', oder 'de' ein.");
            }
        }
        
    }
    //FIRST TIME SETUP
    let cnf_lang = config.lock().unwrap().lang.clone();
    if first_time_setup{
        fllog::log_msg("welcome", cnf_lang.as_str(), "green","black"); //welcome!
        fllog::log_msg("perform_fts", cnf_lang.as_str(), "white","black"); //we will be performing a first time setup
        fllog::log_msg("fts_ipinstructions", cnf_lang.as_str(), "white","black"); //how to set ip address
        fllog::log_msg("fts_get_ip", cnf_lang.as_str(), "yellow","black"); //set ip

        let mut valid = false;//setup for loop
        let regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap(); //writes pegex magic spell that matches for [begin string]X.Y.Z.A[end string] pattern
        while !valid{
            let input = prompt("==> ").replace(" ", ""); //get input, strip away spaces
            let result = regex.captures(input.as_str()); //check if input matches x.y.z.a pattern of IPV4 
            let () = match result {
                Some(_res) => { //if we got any result, it means it matched
                    config.lock().unwrap().set_ip(input); //set IP and write
                    valid = true; //set valid to true, so it will break out of loop
                }
                None => {
                    fllog::log_msg("ip_err", cnf_lang.as_str(), "red","black"); //ip error
                }
              };
        }
    }
    //START APP
    fllog::log_msg("title", cnf_lang.as_str(), "green","black");
    fllog::log_msg("push_enter", cnf_lang.as_str(), "yellow","black");
    let input = prompt("==> "); //app is multilingual. basically, press enter to start app

    if input == "quit"{
        quit();
    }

    let runtime = Runtime::new().unwrap();
    let (vtx_tx, vtx_rx) = mpsc::channel();
    let (ifm_tx, ifm_rx) = mpsc::channel::<String>();

    fllog::log_msg("spawn_mocap", cnf_lang.as_str(), "white","black");
    let m_cnfg = config.clone();
    thread::spawn(move || {
        let ip = m_cnfg.lock().unwrap().ip.clone();
        let lang = m_cnfg.lock().unwrap().lang.clone();
        mocap_bind::mocap_bind(ifm_tx,ip,lang.as_str()).expect("could not bind");
    });
    //let _ = mc.join().expect("Error with IFacialMocap Bind");
    fllog::log_msg("spawn_vts", cnf_lang.as_str(), "white","black");
    let c_cnfg = fsconfig::SharedConfig{shared:config};
    runtime.spawn(async move {
        let lang = c_cnfg.shared.lock().unwrap().lang.clone();
        vts_bind::vts_bind(vtx_rx,&c_cnfg,lang.as_str()).await
    });


    loop{
        let d = ifm_rx.recv().expect("error receiving"); //get from ifacialmocap, block until it gets
        let _ = vtx_tx.send(d); //send data to vts
    }
}
