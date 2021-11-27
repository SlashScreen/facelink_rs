use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Param{
    id:String,
    value:f32
}


pub fn parse_ifm_data(data:&str) -> Vec<Param>{
    let mut out:Vec<Param> = vec!();
    let blendshapes = data.split("|");
    let position_dat_list = [
        "EulerRotationX",
        "EulerRotationY",
        "EulerRotationZ",
        "PositionX",
        "PositionY",
        "PositionZ",
    ];
    for shape in blendshapes {
        if !shape.contains("#") && !shape.contains("&"){
            continue;
        }

        if shape.contains("&"){ //if normal shape
            let mut vals = shape.split("&"); //split into key and value
            let sp = vals.nth(0).unwrap().replace("_","").replace("=","").replace("Left","L").replace("Right","R"); //sanitize key
            let p = Param{ //create parameter
                id : sp, //sanitized shape name
                value : vals.nth(1).unwrap().parse::<f32>().unwrap() //convert value to f32
            };
            out.push(p);//append

        }else if shape.contains("#"){ //or head/eye euler stuff
            let mut vals = shape.split("#"); //split into key and value
            let sp = vals.nth(0).unwrap().replace("_","").replace("=","").replace("Left","L").replace("Right","R");//sanitize key
            let idbase = String::from(sp); //create base for concactenation 

            let angles = vals.nth(1).unwrap().split(",");
            for x in 0..angles.count() { //loop through supplied angles
                let p = Param{
                    id : idbase.clone()+&String::from(position_dat_list[x]), //create key
                    value : vals.nth(1).unwrap().parse::<f32>().unwrap() //value to f32
                };
                out.push(p); //append
            }
        }
    }
    return out;
}
