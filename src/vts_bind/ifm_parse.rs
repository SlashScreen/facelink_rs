//parses iFacialMocap blob

use serde::{Serialize};

//formatted parameter
#[derive(Serialize, Debug)]
pub struct Param{
    id:String,
    value:f32
}


pub fn parse_ifm_data(data:&str) -> Vec<Param>{
    //parse IFM blob and return formatted parameters

    let mut out:Vec<Param> = vec!(); //create output vectors 

    let blendshapes:Vec<&str> = data.split("|").collect(); //split into each parameter by |

    let position_dat_list = [ //just a table for easier euler string concatation
        "EulerRotationX",
        "EulerRotationY",
        "EulerRotationZ",
        "PositionX",
        "PositionY",
        "PositionZ",
    ];

    for shape in blendshapes {
        //for each split blendshape blob
        if !shape.contains("#") && !shape.contains("&"){ //make sure it contains the separators we are looking for
            continue;
        }

        if shape.contains("&"){ //if normal shape
            let vals:Vec<&str> = shape.split("&").collect(); //split into key and value
            let sp = vals[0].replace("_","").replace("=","").replace("Left","L").replace("Right","R"); //sanitize key
            let p = Param{ //create parameter
                id : sp, //sanitized shape name
                value : vals[1].parse::<f32>().unwrap() //convert value to f32
            };
            out.push(p);//append

        }else if shape.contains("#"){ //or head/eye euler stuff
            let vals:Vec<&str> = shape.split("#").collect(); //split into key and value
            let sp = vals[0].replace("_","").replace("=","").replace("Left","L").replace("Right","R");//sanitize key
            let idbase = String::from(sp); //create base for concactenation 

            let angles:Vec<&str> = vals[1].split(",").collect();
            for x in 0..angles.len() { //loop through supplied angles
                let p = Param{
                    id : idbase.clone()+&String::from(position_dat_list[x]), //create key
                    value : angles[x].parse::<f32>().unwrap() //value to f32
                };
                out.push(p); //append
            }
        }
    }
    return out; //return final output
}
