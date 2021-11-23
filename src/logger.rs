use std::collections::HashMap;
use serde_json;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct LoggerMessage{
    id:String,
    translations: HashMap<String,String>
}

#[derive(Deserialize, Debug)]
struct LoggerMessageWrap {
    msgs:Vec<LoggerMessage>
}

fn grab_translation_src(code:&String,lang:&String) -> String {
    let file:String = fs::read_to_string(".\\src\\messages.json").expect("Messages not found!"); //open and read config.json
    let msgw:LoggerMessageWrap = serde_json::from_str(&file).expect("Messages file malformed!"); //parse file
    let msgs = msgw.msgs;
    let msg = msgs.into_iter().find(|x| &x.id == code).unwrap(); //find the message who's code is right. Todo: error handling
    return msg.translations.get(lang).unwrap().to_string(); //return unwrapped string of lang lang
}

pub fn grab_translation(c:&str,l:&str) -> String { //wrapper for gram_translation_src
    return grab_translation_src(&c.to_string(), &l.to_string());
}