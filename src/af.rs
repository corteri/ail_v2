use std::{panic, time::{SystemTime,UNIX_EPOCH}};
use uuid::Uuid;
//all the additional and library functions like TIMESTAMP(),UID(),TOKEN() will exist here.
pub fn time_stamp()->String{
    let mut s_to_send = "".to_string();
    match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n)=>{
            s_to_send = n.as_secs().to_string();
        },
        Err(_)=>{
            panic!("WTF!")
        },
    };
    s_to_send
}

pub fn generate_uid()->String{
    let data_to_send= Uuid::new_v4().to_string();
    data_to_send
}

pub fn count_freq(vector:Vec<&str>,value:&str)->i32{
    let mut c  = 0;
    for i in vector{
        if i == value{
            c = c+1;
        }
    }
    c
}
