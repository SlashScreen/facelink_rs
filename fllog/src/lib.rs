use std::collections::HashMap;
use serde_json;
use serde::Deserialize;
use std::fs;
use std::io::{Write};
use std::str::FromStr;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

pub fn log_msg(msg:&str,lang:&str,fgcol:&str,bgcol:&str){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::from_str(fgcol).unwrap()))); //set color to default white or fgcol
    let _ = stdout.set_color(ColorSpec::new().set_bg(Some(Color::from_str(bgcol).unwrap()))); //set color to default white or fgcol
    let _ = writeln!(&mut stdout, "{}",grab_translation(msg, lang)); //print string
}
