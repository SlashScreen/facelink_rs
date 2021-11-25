use std::collections::HashMap;

use tungstenite::{connect, Message};
use url::Url;
use serde_json;
use serde_json::Value;
use serde::{Deserialize,Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type AuthResult<T> = std::result::Result<T,str>;
type AuthError = dyn std::error::Error;


#[derive(Serialize, Debug)]
struct ApiRequest{
    apiName:String,
    apiVersion:String,
    requestID:String,
    messageType:String,
    data:HashMap<String,String>
}



async fn ping(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>){
    let req = ApiRequest{
        apiName:String::from("VTubeStudioPublicAPI"),
        apiVersion:String::from("1.0"),
        requestID:String::from("FaceLinkRS"),
        messageType:String::from("APIStateRequest"),
        data:HashMap::new()
    };

    sock.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap();
    loop {
        let msg = sock.read_message().expect("Error reading message");
        println!("Received: {}", msg);
        if msg.into_text().unwrap() != ""{
            return;
        }
        
    }
}

async fn try_get_auth_token(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>) -> String{
    let req = ApiRequest{ //API request for trying to authenticate app
        apiName:String::from("VTubeStudioPublicAPI"),
        apiVersion:String::from("1.0"),
        requestID:String::from("FaceLinkRS"),
        messageType:String::from("AuthenticationTokenRequest"),
        data:HashMap::from([
            (String::from("pluginName"),String::from("FacelinkRS")),
            (String::from("pluginDeveloper"),String::from("Slashscreen"))
        ])
    };

    sock.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap(); //send request

    let data:Value;
    loop { //wait until response
        let msg = sock.read_message().expect("Error reading message"); 
        if msg.to_text().unwrap() != ""{
            println!("Received: {}", msg);
            data = serde_json::from_str(msg.to_text().unwrap()).unwrap();
            break;
        }
    }

    if data["messageType"] == "AuthenticationTokenResponse"{ //if success
        return data["data"]["authenticationToken"].to_string(); //return token
    }else{ //if not
        return String::from("nil"); //return "nil"
    }

}

async fn get_auth(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,token:&mut str) -> bool{
    let tk:&str;
    if token == ""{
        println!("trying to get key...");
        let mut res = try_get_auth_token(sock).await;
        if res == "nil"{
            return false;
        }else{
            return true;
            //tk = res.as_mut_str();
        }
    }else{
        let req = ApiRequest{ //API request for trying to authenticate app
            apiName:String::from("VTubeStudioPublicAPI"),
            apiVersion:String::from("1.0"),
            requestID:String::from("FaceLinkRS"),
            messageType:String::from("AuthenticationRequest"),
            data:HashMap::from([
                (String::from("pluginName"),String::from("FacelinkRS")),
                (String::from("pluginDeveloper"),String::from("Slashscreen")),
                (String::from("authenticationToken"),String::from(token))
            ])
        };

        sock.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap(); //send request

        let data:Value;
        loop { //wait until response
            let msg = sock.read_message().expect("Error reading message"); 
            if msg.to_text().unwrap() != ""{
                println!("Received: {}", msg);
                data = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                break;
            }
        }

        println!("{}",data["data"]["authenticated"].as_bool().unwrap());
        if data["data"]["authenticated"].as_bool().unwrap() {
            return true;
        }else{
            return false;
        }
        

    }
    
}



pub async fn vts_bind(token:&mut str){
    //todo: API port configure
    let (mut socket, response) = connect(Url::parse("ws://localhost:8001").unwrap()).expect("Can't connect"); //connect to vts localhost
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    
    ping(&mut socket).await;
    let authres = get_auth(&mut socket,token).await;
    if authres{
        println!("auth'd");

    }else{
        println!("unauthd");
    }
    
    return;
}
