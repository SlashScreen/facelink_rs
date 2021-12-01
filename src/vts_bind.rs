//handles all interaction with vtube studio

//include standard
use std::collections::HashMap;
use std::fs;
//include vendor
use tungstenite::{connect, Message};
use url::Url;
use serde_json;
use serde_json::Value;
use serde::{Deserialize,Serialize};
use fsconfig;
//include modules
mod ifm_parse;

//structs for requests
//lots of repeated code here, could probably do to make a single ApiRequest and figure out how to let data be anything. serde is strictly typed though, i think, so that may be harder than in Go where i can just use interface{}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct ApiRequest{
    apiName:String,
    apiVersion:String,
    requestID:String,
    messageType:String,
    data:HashMap<String,String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct ApiInjectRequest{
    apiName:String,
    apiVersion:String,
    requestID:String,
    messageType:String,
    data:HashMap<String,Vec<ifm_parse::Param>>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct ApiCreationRequest{
    apiName:String,
    apiVersion:String,
    requestID:String,
    messageType:String,
    data:ApiParam
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct ApiParam{
    parameterName:String,
    explanation:String,
    min:f32,
    max:f32,
    defaultValue:f32
}

//parameter from the json file

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Parameter{
    GUID:String,
    Type:String,
    ID:String,
    Name:String,
    Group:String,
    #[serde(rename(deserialize = "min_val"))]
    min_val:f32,
    #[serde(rename(deserialize = "def_val"))]
    def_val:f32,
    #[serde(rename(deserialize = "min_val"))]
    max_val:f32,
    Repetition:bool,
    Description:String
}



async fn ping(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>){
    //keeps connection alive 
    let req = ApiRequest{ //create ping request
        apiName:String::from("VTubeStudioPublicAPI"),
        apiVersion:String::from("1.0"),
        requestID:String::from("FaceLinkRS"),
        messageType:String::from("APIStateRequest"),
        data:HashMap::new()
    };

    sock.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap(); //send ping

    loop { //wait for response
        let msg = sock.read_message().expect("Error reading message");
        if msg.into_text().unwrap() != ""{
            return;
        }
        
    }
}

async fn try_get_auth_token(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>) -> String{
    //attempts to get a new token from vts
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

async fn first_time_setup(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>){
    //read parameters file and create parameters within vts
    let file:String = fs::read_to_string(".\\src\\config.json").expect("Config not found!"); //open and read config.json
    let pms:Vec<Parameter> = serde_json::from_str(&file).expect("Config file malformed!"); //parse file

    for pm in pms{
        let req = ApiCreationRequest{ //create parameter creation request
            apiName:String::from("VTubeStudioPublicAPI"),
            apiVersion:String::from("1.0"),
            requestID:String::from("FaceLinkRS"),
            messageType:String::from("ParameterCreationRequest"),
            data:ApiParam{
                parameterName:String::from(pm.Name),
                explanation:String::from(pm.Description),
                min:pm.min_val,
                max:pm.max_val,
                defaultValue:pm.def_val
            }
        };

        //send request
        sock.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap();

        //wait for response as not to overload VTS
        loop{
            let msg = sock.read_message().expect("Error reading message"); 
            if msg.to_text().unwrap() != ""{
                break;
            }
        }
    }
}

async fn process_token_response(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, res:String,cnfg:&fsconfig::SharedConfig) -> bool{
    //some reused code to check if we've been rejected

    if res.as_str() == "nil"{
        return false; //user rejected us :(
    }else{
        cnfg.set_token(res);
        first_time_setup(sock).await; //perform first time setup
        return true;
        //save token
    }
}

async fn get_auth(sock:&mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,token:&str,cnfg:&fsconfig::SharedConfig) -> bool{
    //attempts to find authorization state from vtube studio

    //let tk:&str;
    if token == ""{ //if we've never used the app before
        println!("trying to get key...");
        let res = try_get_auth_token(sock).await;
        return process_token_response(sock, res,cnfg).await;
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
        loop { //wait for response
            let msg = sock.read_message().expect("Error reading message"); 
            if msg.to_text().unwrap() != ""{
                //println!("Received: {}", msg);
                data = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                break;
            }
        }

        println!("{}",data["data"]["authenticated"].as_bool().unwrap()); //print authentication
        if data["data"]["authenticated"].as_bool().unwrap() { //if we are authenticated
            return true;
        }else{ //if not
            let res = try_get_auth_token(sock).await; //ask vtube studio for auth token
            return process_token_response(sock, res,cnfg).await;
        }
        

    }
    
}



pub async fn vts_bind(rc:std::sync::mpsc::Receiver<String>,cnfg:&fsconfig::SharedConfig) {
    //binds to vts and forwards input from iFacialMocap
    
    let (mut socket, response) = connect(Url::parse("ws://localhost:8001").unwrap()).expect("Can't connect"); //connect to vts localhost
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    
    ping(&mut socket).await; //ping socket
    
    
    let authres = get_auth(&mut socket, &cnfg.get_token(),cnfg).await;
    if authres{
        println!("auth'd");
        //now working
        loop{
            let d = rc.recv().unwrap(); //get data from channel (from iFacialMocap)
            let params = ifm_parse::parse_ifm_data(d.as_str()); //parse blob
            let req = ApiInjectRequest{ //create injection request
                apiName:String::from("VTubeStudioPublicAPI"),
                apiVersion:String::from("1.0"),
                requestID:String::from("FaceLinkRS"),
                messageType:String::from("InjectParameterDataRequest"),
                data:HashMap::from([
                    (String::from("parameterValues"),params) //parameters
                ])
            };

            socket.write_message(Message::Text(serde_json::to_string(&req).unwrap().as_str().into())).unwrap(); //send injection request
        }

    }else{
        println!("unauthd");
    }
    
    return;
}
