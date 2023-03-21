use tide::{Request,Response,Result};
use serde::{Deserialize,Serialize};
use serde_json::{to_string as to_json_string};


#[derive(Serialize,Deserialize)]
pub struct Message<'msg>{
    message:&'msg str
}

pub async fn hello_client(_req:Request<()>)->Result{
    let mut res= Response::new(200);
    let res_json=to_json_string(&Message{message:"Welcome to 0xCrypt00o Company"})?;
    res.set_content_type("application/json");
    res.set_body(res_json);
    return Ok(res);
}
