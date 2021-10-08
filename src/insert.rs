//pub  static mut COLUMN:Vec<String> = vec![];
//pub static mut VALUES:Vec<Vec<String>> = vec![];
//Add The Data Inside The Main Portfolio
use std::{panic, time::{SystemTime,UNIX_EPOCH}, vec};
use uuid::Uuid;

pub fn insert_column(data:Vec<&str>)-> Vec<String>{
    //What we have to do is to create the object for this class
    let mut data_to_send = vec![];
    for i in data{
        data_to_send.push(i.trim().trim().to_string());
    }
    data_to_send
}

pub fn insert_new(data_model:Vec<&str>,column:Vec<String>, value:Vec<Vec<String>>,input:Vec<(String,String)>)->(bool,Vec<Vec<String>>){
    let mut check = true;
    //let mut column_to_send = vec![];
    let mut value_to_send = value;
    let mut keys = vec![];
    let mut keys_values = vec![];

    for i in 0..data_model.len(){
        let  o_str = &data_model[i].trim().trim_end();
        //let mut  data_str:Vec<&str> = data_model[i].split(':').collect();
       let o_str = &o_str.replace('{'," ");
        let o_str = &o_str.replace('}'," ");
        let data_str:Vec<&str> = o_str.trim().trim_end().split(',').collect();
        let mut ts = "".to_string();
        for j in data_str{
            let mut vec_data:Vec<&str> = j.trim().trim_end().split(':').collect();
            keys.push(vec_data[0].to_string());
            //this is the place from where we can access values
            if vec_data[1].len()>9{
                if vec_data[1][0..10].to_string() == "TIMESTAMP("{
                    ts = time_stamp();
                    vec_data[1] = &ts;
                }
                else if vec_data[1][0..5].to_string() == "UID()"{
                    ts = generate_uid();
                    vec_data[1] = &ts;
                }
                else if vec_data[1][0..6].to_string() == "INPUT("{
                    ts = vec_data[1].replace("INPUT(","");
                    ts = ts.replace(")","");
                    ts = give_value(ts,input.clone());
                    vec_data[1] = &ts;
                }
            }
            else if vec_data[1].len()>4{
                if vec_data[1][0..5].to_string() == "UID()"{
                    ts = generate_uid();
                    vec_data[1] = &ts;
                }
                else if vec_data[1].len()>5{
                    if vec_data[1][0..6].to_string() == "INPUT("{
                        ts = vec_data[1].replace("INPUT(","");
                        ts = ts.replace(")","");
                        ts = give_value(ts,input.clone());
                        vec_data[1] = &ts;
                    }
                }
            }
            keys_values.push(vec_data[1].to_string());
        }
    }

    //match the columns
    for i in 0..keys_values.len(){
        let index = check_column(keys[i].clone(), column.clone());
        if index>=0{
            if value_to_send.len()==0{
            value_to_send.push(vec![keys_values[i].clone()]);
            }
            else if value_to_send.len() == index as usize{
//                value_to_send[index as usize].push(keys_values[i].clone());
            value_to_send.push(vec![keys_values[i].clone()]);

            }
            else{
                value_to_send[index as usize].push(keys_values[i].clone());
            }
        }
        else{
            check = false;
            break;
        } 
    }

    //match the column and find the truth

    if column.len()>keys.len(){
        for i in 0..column.len(){
            //
            let mut check = false;

            for j in 0..keys.len(){
                if keys[j] == column[i]{
                    check = true;
                    break;
                }
            }
            if !check{
                if value_to_send.len() == i {
                    value_to_send.push(vec!["".to_string()]);
                }
                else if value_to_send.len() == 0 {
                    value_to_send.push(vec!["".to_string()]);
                }
                else{
                    value_to_send[i].push("".to_string());
                }
            }
        }
    }
    (check,value_to_send)
}

fn check_column(column_value:String,column:Vec<String>)->i32{

    let mut check = false;
    let mut index = 0;
    //println!("{}",column.len()-1);
    for i in 0..column.len(){
        //println!("if {} == {}",column[i],column_value);
        if column[i] == column_value{
            index = i;
            check = true;
            break;
        }
    }

    if check{
        index as i32
    }
    else{
        -1
    }

}

 fn time_stamp()->String{
    let mut s_to_send = "".to_string();
    match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n)=>{
            s_to_send = n.as_secs().to_string();
        },
        Err(_)=>{
            panic!("Wr")
        },
    };
    s_to_send
}
fn generate_uid()->String{
    let data_to_send= Uuid::new_v4().to_string();
    data_to_send
}

fn give_value(key:String,vec:Vec<(String,String)>)->String{
        let mut string_to_send = String::from("");
        for i in vec{
            if i.0 == key{
                string_to_send = i.1;
                break;
            }
        }
        string_to_send
}