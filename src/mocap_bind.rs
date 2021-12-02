use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::str;

use fllog;


pub fn mocap_bind(sd:std::sync::mpsc::Sender<String>,ip:String,lang:&str) -> std::io::Result<()> {
    //Connect to iFacialMocap
    let mode = "udp";
    if mode == "udp"{
        fllog::log_msg("mocap_connect", lang, "white","black");
        let sock = UdpSocket::bind("0.0.0.0:49983")?; //connect
        let buf = b"iFacialMocap_sahuasouryya9218sauhuiayeta91555dy3719|sendDataVersion=v2"; //create connecting packet
        sock.send_to(buf, format!("{}:49983",ip))?;  //send packet
        loop{
            let mut d = [0;1024]; //get buffer
            let _ = sock.recv_from(&mut d)?; //receive
            let _ = sd.send(String::from(str::from_utf8(&d).unwrap())); //send to main thread
        }
    }else{
        //not working
        let mut stream = TcpStream::connect(format!("{}:49983",ip))?;
        stream.write(b"iFacialMocap_UDPTCP_sahuasouryya9218sauhuiayeta91555dy3719|sendDataVersion=v2")?;
        loop{
            let mut d = String::from("");
            let _ = stream.read_to_string(&mut d)?;
            let _ = sd.send(d);
        
        }
    }
} 
