use core::panic;
use std::{vec};
use std::env::{self};
use std::io;
use std::fs;
mod data_model;
mod insert;
mod af;


static mut KEYS:Vec<String> = vec![];
static mut VALUES:Vec<Vec<String>> = vec![];
static mut VECTORS:Vec<Vec<String>> = vec![];
static mut LOCATION_STORAGE:Vec<Vec<String>> = vec![];
static mut LOCATION:Vec<(String,usize)> = vec![];
static  mut INPUT_STORAGE:Vec<(String,String)> = vec![];
static  mut FUNCTION:Vec<(String,String)> = vec![];
static  mut FUNCR:Vec<(String,String,Vec<Vec<String>>)> = vec![];

static  mut STORAGE:Vec<String> = vec![];
static  mut CLASS:Vec<(String,String)> = vec![];
static  mut OBJECTS:Vec<(String,(Vec<String>,Vec<Vec<String>>))> = vec![];

/*
1.IMPLEMENT FOR THE ' ' THIS SYMBOLS IN OUR PROGRAME FOR EVERY ITERATIONS.

*/



fn main() {
   //let querys:Vec<String> = vec!["GET FROM abc where name eq n1 AND age neq 1 > STORE ([{name:<c<name}|{naam:Adi}])".to_string(),"GET FROM abc where name eq AXC(naam) AND age eq 2 ".to_string(),"INSERT [{name:Adi,age:2}]".to_string()];
   println!("Alioth 1.0");

  // let mut query = String::from("");
   let args:Vec<String> = env::args().collect();
   let querys_ = fs::read_to_string(args[1].clone()).expect("Something Wrong With The File");
    function_executer(querys_);
   // instruction_executer(querys_);
  
}

fn dml(query:String,index:usize)->Vec<String>{
    //this has a some error like when we write name eq n1 AND name eq n2;
   //let query = String::from("Get From abc where name eq n1 AND age eq 2");

   
   let all_query:Vec<&str> = query.split(' ').collect();
   
   //let hello_data:(Vec<String>,Vec<String>) = get_single_vectors(data);
   //println!("keys {:?} and Values {:?}", data.0,data.1);
   //let data = insert::get_data();
   //let data:(Vec<String>,Vec<Vec<String>>) = data_model::send_model2();

let mut data = (vec![],vec![]);


   unsafe{
    
       data = OBJECTS[index].clone().1;
       KEYS = data.clone().0;
       VALUES = data.clone().1;
      // println!("keys {:?} values{:?}",KEYS,VALUES);
   }


   //Separate The Tokens Now.

   //we have to implement the IDEA FOR STRING QUOTE.

   //Separate All The Tokens

   
   
   let mut conditional_operator:Vec<&str> = vec![];
   let mut relational_operator:Vec<&str> = vec![];
   let mut keys:Vec<String> = vec![];
   let mut values:Vec<String> = vec![];
   let mut data_collector:Vec<String> = vec![];
   let mut data_to_send:Vec<String> = vec![];

   for i in 4..all_query.len()-1{

    //this loop is separating the tokens from the query;
    /*
    1.OR
    2.AND
    3.eq
    4.neq
     */
       match all_query[i]{
           "AND"=> conditional_operator.push(all_query[i]),
           "OR"=>conditional_operator.push(all_query[i]),
           "eq"=>
           {
            relational_operator.push(all_query[i]);
            keys.push(all_query[i-1].replace("!@#880"," "));
            values.push(all_query[i+1].replace("!@#880"," "));
           },
           "neq"=>
           {
            relational_operator.push(all_query[i]);
            keys.push(all_query[i-1].replace("!@#880"," "));
            values.push(all_query[i+1].replace("!@#880"," "));
           },
           _=>print!(" "),
       };
   }

   //CHECK FOR CONCATATION AND HASHOF WILL BE IMPLEMENTED HERE

   for i in 0..values.len(){
       if values[i].len()>7{
           if values[i][0..8].to_string() == "CONCATE("{
            let new_string = concate(values[i].clone());
            values[i] = new_string;
           }
           else if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
            let mut new_string = values[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             values[i] = new_string;
        } else if values[i][0..4].to_string() == "AFV("{
            values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
        }
           else if values[i].len()>9{
               if values[i][0..10].to_string() == "TIMESTAMP("{
                   let new_value = af::time_stamp();
                   values[i] = new_value;
               }
              
           }
       }
       else if values[i].len()>4{
           if values[i][0..5].to_string() == "UID()"{
               let new_string = af::generate_uid();
               values[i] = new_string;
           }
           else if values[i][0..4].to_string() == "AFV("{
               values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
           }
           else if values[i].len()>5{
               if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
                   let mut new_string = values[i].replace("INPUT(","");
                    new_string = new_string.replace(")","");
                    new_string = give_value(new_string);
                    values[i] = new_string;
               }
           }
       }
       
       if keys[i].len()>7{
           if keys[i][0..8].to_string() == "CONCATE("{
               let new_key = concate(keys[i].clone());
               keys[i] = new_key;
           }
           else if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
            let mut new_string = keys[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             keys[i] = new_string;
        }else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(keys[i][4..keys[i].len()-1].to_string()).1;
        }
           else if keys[i].len()>9{
            if keys[i][0..10].to_string() == "TIMESTAMP("{
                let new_key = af::time_stamp();
                keys[i] = new_key;
            }
            
        }
       }
       else if keys[i].len()>4{
        if keys[i][0..5].to_string() == "UID()"{
            let new_string = af::generate_uid();
            keys[i] = new_string;
        }else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(keys[i][4..keys[i].len()-1].to_string()).1;
        }
        else if keys[i].len()>5{
            if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                let mut new_string = keys[i].replace("INPUT(","");
                 new_string = new_string.replace(")","");
                 new_string = give_value(new_string);
                 keys[i] = new_string;
            }
        }
    }
   }

   //Access TO The Values


   for i in 0..values.len(){
       if values[i].len()>4{
        let slice =  values[i][0..4].to_string();
        if slice == "AXC("{
         let mut real_value = values[i][3..].replace("("," ");
         real_value = real_value.replace(")"," ");
         real_value = real_value.trim().trim_end().to_string();
         //access the real value
         real_value = access_value(real_value);
         values[i] = real_value;
 
        }
       }
       
   }


   //println!("{:?}",values);
   let   key_check = cmp_for_keys(keys.clone());
   let mut key_index = vec![];
   if key_check.0{
       key_index = key_check.1;
   }
   else{
    println!("error column not matched");
    return vec![]
   }
   let mut conditional_counter:i32 = -1;
   let mut i:usize = 0;

  // println!("{:?}",key_index);
  if relational_operator.len() == 0{
    unsafe{
    // for i in 0..VALUES.len(){
         for j in 0..VALUES[i].len(){
             let str_t = generate_str(j);
             data_to_send.push(str_t);
         }
     //}
    }
}
  else if relational_operator.len() == 1{
    if relational_operator[0] == "eq"{
        if data.1.len()>0{
            for k in 0..data.clone().1[0].len(){
                if data.clone().1[key_index[i]][k] == values[i].clone(){
                    let simple_str = generate_str(k);
                    data_to_send.push(simple_str);
                }
            }
        }
        
    }
    else if relational_operator[0] == "neq"{
        if data.1.len()>0{
        for k in 0..data.clone().1[0].len(){
            if data.clone().1[key_index[i]][k] != values[i].clone(){
                let simple_str = generate_str(k);
                data_to_send.push(simple_str);
            }
        }
    }
    }
    else{
        //throw an error.
    }
}
else{

    while i<relational_operator.len(){
        if i != relational_operator.len(){
            conditional_counter = conditional_counter+1;
    
        }   
    
           if conditional_counter == 0{
           if relational_operator[i] == "eq"{
               if relational_operator[i+1]=="eq"{
                if conditional_operator[conditional_counter as usize] == "AND"{
                 
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() && data.clone().1[key_index[i+1]][k] == values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
    
                    }
                }
                else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() || data.clone().1[key_index[i+1]][k] == values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
                        //do something
                    
                }
                else{
                    //throw an error.
                }
               }
               else if relational_operator[i+1]=="neq" {
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() && data.clone().1[key_index[i+1]][k] != values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
    
                    }
                // }
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() || data.clone().1[key_index[i+1]][k] != values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
                   }
               }
               else{
                   //throw an error.
               }
           }
           else if relational_operator[i] == "neq"{
               if relational_operator[i+1] == "neq"{
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() && data.clone().1[key_index[i+1]][k] != values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
    
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() || data.clone().1[key_index[i+1]][k] != values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
                   }
                   else{
                       //throw an error.
                   }
               }
               else if relational_operator[i+1] == "eq"{
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() && data.clone().1[key_index[i+1]][k] == values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() || data.clone().1[key_index[i+1]][k] == values[i+1]{
                            let simple_str = generate_str(k);
                            data_to_send.push(simple_str);
                        }
                    }
                   }
                   else{
                       //throw an error Operators other than AND and OR.
                   }
               }
               else{
                   //throw an error Operators Other Than eq and neq.
               }
           }
           else{
               //throw an error
           }
           //how to increase the
           
           //i.checked_add(2).unwrap();
        }
        else {
       
            
            if relational_operator[i] == "eq"{
                for k in 0..data.clone().1[0].len(){
                    if data.clone().1[key_index[i]][k] == values[i].clone(){
                        let simple_str = generate_str(k);
                        data_to_send.push(simple_str);
                    }
                }
            }
            else if relational_operator[i] == "neq"{
                for k in 0..data.clone().1[0].len(){
                    if data.clone().1[key_index[i]][k] != values[i].clone(){
                        let simple_str = generate_str(k);
                        data_to_send.push(simple_str);
                    }
                }
            }
            else{
                //throw an error.
            }
        }
           if conditional_counter>0{
               //do this
    
               if conditional_operator[conditional_counter as usize] == "AND"{
                   //call the set union 
                  
                   data_to_send = and_algo(data_to_send.clone(),data_collector.clone());
                   data_to_send = remove_duplicate(data_to_send);
                   data_collector = vec![];
               }
               else if conditional_operator[conditional_counter as usize] == "OR"{
                  //merge both of the sets
                  data_to_send = or_algo(data_to_send.clone(),data_collector.clone());
                  data_to_send = remove_duplicate(data_to_send);
                  data_collector = vec![];
               }
               else{
                   println!("Throw An Error");
               }
           }
        // conditional_counter = conditional_counter+1;
       
        if i == 0{
            i = i+2;
        }
        else{
            i = i+1;
    
        }
        
    
    
       }
}
   //we will create the conditions chain for the rest of the programs
  // println!("{:?}",data_to_send);
   data_to_send
}
 





fn cmp_for_keys(keys:Vec<String>)->(bool,Vec<usize>){

    unsafe{
        let mut keys_index_to_send = vec![];
        let mut check = false;
        let mut emr = false;
       
        for i in 0..keys.len(){
            check = false;
            for j in 0..KEYS.len(){
                if keys[i] == KEYS[j]{
                    check = true;
                    keys_index_to_send.push(j);
                    break;
                }
            }
            if !check{
                emr = true;
                break;
            }
        }
              /* 
               if !check{
                emr = true;
                break;
               }
               */
            
        
        if emr{
            (false,vec![])
        }
        else{
            
            (true,keys_index_to_send)
        }
    }
}



fn and_algo(all_vec:Vec<String>,collected_vec:Vec<String>)->Vec<String>{
    let mut vec_to_send = vec![];
    for i in collected_vec{
        match all_vec.binary_search(&i) {
            Ok(_j)=>{ vec_to_send.push(i.to_string())},
            Err(_)=>{},
        };
    }
    
    vec_to_send
    //vec_to_send = vec_to_send.dedup();
}

fn and_algo_usize(all_vec:Vec<usize>,collected_vec:Vec<usize>)->Vec<usize>{
    println!("and also {:?}  collected vec {:?}",all_vec,collected_vec);
    let mut vec_to_send = vec![];
    for i in collected_vec{
        match all_vec.binary_search(&i) {
            Ok(_j)=>{ vec_to_send.push(i)},
            Err(_)=>{},
        };
    }
   
    vec_to_send
    //vec_to_send = vec_to_send.dedup();
}


fn or_algo(mut all_vec:Vec<String>,mut collected_vec:Vec<String>)->Vec<String>{

    all_vec.append(&mut collected_vec);
    all_vec
}

fn or_algo_usize(mut all_vec:Vec<usize>,mut collected_vec:Vec<usize>)->Vec<usize>{

    all_vec.append(&mut collected_vec);
    all_vec
}


fn remove_duplicate(mut all_data:Vec<String>)->Vec<String>{
 //convert all the data into its true form

 for i in 0..all_data.len(){
    for j in i..all_data.len()-1{
        if all_data[j] == all_data[i]{
            if i !=j{
            all_data.remove(j);
        }
    }
    }
 }
 all_data
}

fn remove_duplicate_usize(mut all_data:Vec<usize>)->Vec<usize>{
    //convert all the data into its true form
    for i in 0..all_data.len(){
       for j in i..all_data.len()-1{
           if all_data[j] == all_data[i]{
               if i !=j{
               all_data.remove(j);
           }
       }
       }
    }
    all_data
   }


fn generate_str(index:usize)->String{
    unsafe{
        let mut string_to_send = "".to_string();
        for i in 0..KEYS.len(){
            let keys = &KEYS[i];
            let values = &VALUES[i][index];
            if i != KEYS.len()-1{
                string_to_send.push_str(&keys);
                string_to_send.push_str(":");
                string_to_send.push_str(&values);
                string_to_send.push_str(",");


            }
            else{
                string_to_send.push_str(&keys);
                string_to_send.push_str(":");
                string_to_send.push_str(&values);
            }            
        }
        //now from here start the replacing stuffs
        
        string_to_send.insert_str(0, "{");
        string_to_send.insert_str(string_to_send.len(), "}");
        string_to_send
    }
}

//check at store
fn store(store_query:&str){
    let  data_to = &store_query.replace("([","").to_string();
      let  data_to = data_to.replace("])","").to_string();
      //let  data_to = data_to.replace(")"," ").to_string();
      //let  data_to = data_to.replace("]"," ").to_string();
      let data_to = data_to.trim().trim().to_string().replace("[","").replace("]","");
      let data_to = data_to.trim_end();

        let values:Vec<&str> =  data_to.split("|").collect();
        for i in values{
            let check_i = i.replace("{"," ");
            let check_i = check_i.replace("}"," ");
            let  check_i = check_i.trim();
            let  check_i = check_i.trim_end();
       
           
                let mut key_value:Vec<&str> = check_i.split(':').collect();
                //check for concate
                let mut new_value = String::from("");
                if key_value[1].len()>7{
                     if key_value[1][0..8].to_string() == "CONCATE("{
                    new_value = concate(key_value[1].to_string());
                   key_value[1] = &new_value;
               }
               else if key_value[1][0..4].to_string() == "AXC("{
                new_value = key_value[1].replace("AXC(","");
                new_value = new_value.replace(")","");
                new_value = access_value(new_value.trim().trim_end().to_string());
                key_value[1] = &new_value;
               }
               else if key_value[1][0..6].to_string() == "UID()"{
                new_value = af::generate_uid();
                key_value[1] = &new_value;
               }
               else if key_value[1][0..4].to_string() == "AFV("{
                new_value = afv(key_value[1][4..key_value[1].len()-1].to_string()).1;
                key_value[1] = &new_value;
            }
               else if key_value[1][0..6].to_string() == "INPUT("{
                       
                // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                      new_value = key_value[1].replace("INPUT(","");
                      new_value = new_value.replace(")","");
                      new_value = give_value(new_value);
                      key_value[1] = &new_value;
         }
               else if key_value[1].len()>9{
                if key_value[1][0..10].to_string() == "TIMESTAMP("{
                    new_value = af::time_stamp();
                    key_value[1] = &new_value;
                }
            }
            }
            else if key_value[1].len()>3{
                 if key_value[1][0..4].to_string() == "AXC("{
                    //we have to create a variable to access the values;
                    new_value = key_value[1].replace("AXC(","");
                    new_value = new_value.replace(")","");
                    new_value = access_value(new_value.trim().trim_end().to_string());
                    key_value[1] = &new_value;
                }
                else if key_value[1].len()>4{
                    if key_value[1][0..5].to_string() == "UID()"{
                        new_value = af::generate_uid();
                        key_value[1] = &new_value;
                    }
                    else if key_value[1][0..4].to_string() == "AFV("{
                        new_value = afv(key_value[1][4..key_value[1].len()-1].to_string()).1;
                        key_value[1] = &new_value;
                    }
                    else if key_value[1].len()>5{
                        if key_value[1][0..6].to_string() == "INPUT("{
                       
                            // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                                  new_value = key_value[1].replace("INPUT(","");
                                  new_value = new_value.replace(")","");
                                  new_value = give_value(new_value);
                                  key_value[1] = &new_value;
                     }
                    } 
                }
            }
               
                //GET THE KEY INDEX AND SHOW THEIR VALUES.
                let split_check:Vec<&str> = key_value[1].split('<').collect();
                if split_check.len()>1{
                    //println!("{:?} splitted value",split_check);
                    if split_check[1] == "c"{
                         
                        unsafe{

                        let current = VECTORS.len()-1 as usize;
                        let get = to_get(VECTORS[current].clone(),split_check[2]);
                        LOCATION_STORAGE.push(get);
                        LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                        }
                        }
                    else{
                        println!("IT IS NOT CURRENT VALUE");
                       unsafe{
                        let current = split_check[1].to_string().parse::<i32>().unwrap() as usize;
                        if current<VECTORS.len(){
                        let get = to_get(VECTORS[current].clone(),split_check[2]);
                        LOCATION_STORAGE.push(get);
                        LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                        }
                        else{
                            //throw error;
                        } 
                       }
                        
                    }
                }
                else{
                    //normal value
                    let put = vec![key_value[1].to_string()];
                    unsafe{
                        LOCATION_STORAGE.push(put);
                        LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                    }
                }
                

            
        }   
}

fn to_get(data:Vec<String>,keys:&str)->Vec<String>{
    let mut vecl:Vec<String> = vec![];
    for i in data{
     let d = i.replace("{"," ");
     let mut d = d.replace("}"," ");
     d = d.trim().to_string();
     d = d.trim_end().to_string();
     let values:Vec<&str> = d.split(",").collect();
     for j in values{
         let keys_values:Vec<&str> = j.split(":").collect();
         if keys_values[0] == keys{
             vecl.push(keys_values[1].to_string());
         }
     } 
 }

 vecl
}

pub fn access_value(key:String)->String{
    unsafe{
        let mut index:usize = 0;
        let mut found = false;
        for i in 0..LOCATION.len(){
            if key == LOCATION[i].0{
                index = LOCATION[i].1;
                found = true;
                break;
            }
        }
        if found{
                let real_value = LOCATION_STORAGE[index].clone();
                real_value[real_value.len()-1 as usize].clone()
        }
        else{
            "".to_string()
        }
    }
}



//cmp 



 fn cmp_fn(query:String)->Vec<String>{
    //split the query
let mut conditional_operator:Vec<String> = vec![];
   let mut relational_operator:Vec<String> = vec![];
   let mut keys:Vec<String> = vec![];
   let mut values:Vec<String> = vec![];
   let mut data_collector:Vec<String> = vec![];
   let mut data_to_send:Vec<String> = vec![];
    let spliter:Vec<&str> = query.trim().trim_end().split(" ").collect();
    for i in 0..spliter.len(){
        match spliter[i] {
            "eq"=>{
                relational_operator.push(spliter[i].to_string());
                keys.push(spliter[i-1].replace("!@#880"," "));
                values.push(spliter[i+1].replace("!@#880"," "));
            },
            "neq"=>{
                relational_operator.push(spliter[i].to_string());
                keys.push(spliter[i-1].replace("!@#880"," "));
                values.push(spliter[i+1].replace("!@#880"," "));
            },
            "OR"=>{
                conditional_operator.push(spliter[i].to_string());
            },
            "AND"=>{
                conditional_operator.push(spliter[i].to_string());
            },
            _=>{

            }

        }
    }
   //CHECK FOR CONCATATION AND HASHOF WILL BE IMPLEMENTED HERE
   for i in 0..values.len(){
    if values[i].len()>7{
        if values[i][0..8].to_string() == "CONCATE("{
         let new_string = concate(values[i].clone());
         values[i] = new_string;
        }
        else  if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
            let mut new_string = values[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             values[i] = new_string;
        } else if values[i][0..4].to_string() == "AFV("{
            values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
        }
        else if values[i].len()>9{
            if values[i][0..10].to_string() == "TIMESTAMP("{
                let new_value = af::time_stamp();
                values[i] = new_value;
            }
           
        }
    }
    else if values[i].len()>4{
        if values[i][0..5].to_string() == "UID()"{
            let new_string = af::generate_uid();
            values[i] = new_string;
        }
        else if values[i][0..4].to_string() == "AFV("{
            values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
        }
        else if values[i].len()>5{
            if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
                let mut new_string = values[i].replace("INPUT(","");
                 new_string = new_string.replace(")","");
                 new_string = give_value(new_string);
                 values[i] = new_string;
            }
        }
    } 
    if keys[i].len()>7{
        if keys[i][0..8].to_string() == "CONCATE("{
            let new_key = concate(keys[i].clone());
            keys[i] = new_key;
        }
        else if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
            let mut new_string = keys[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             keys[i] = new_string;
        }
        else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(keys[i][4..keys[i].len()-1].to_string()).1;
        }
        else if keys[i].len()>9{
            if keys[i][0..10].to_string() == "TIMESTAMP("{
                let new_key = af::time_stamp();
                keys[i] = new_key;
            }
            
        }
    }
    else if keys[i].len()>4{
        if keys[i][0..5].to_string() == "UID()"{
            let new_string = af::generate_uid();
            keys[i] = new_string;
        }
        else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(keys[i][4..keys[i].len()-1].to_string()).1;
        }
        else if keys[i].len()>5{
            if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                let mut new_string = keys[i].replace("INPUT(","");
                 new_string = new_string.replace(")","");
                 new_string = give_value(new_string);
                 keys[i] = new_string;
            }
        }
    }
}

//generating the stuff as our own.
    for i in 0..values.len(){
        if values[i].len()>=4 && keys[i].len()>=4{   
         let slice =  values[i][0..4].to_string();
         let key_slice = keys[i][0..4].to_string();
       //  println!("{:?} AND {:?}",slice,key_slice);
         if slice == "AXC(" && key_slice == "AXC("{
            // println!("We ARE IN FIRST IF");
          let mut real_value = values[i][3..].replace("("," ");
          real_value = real_value.replace(")"," ");
          real_value = real_value.trim().trim_end().to_string();
          //access the real value
          real_value = access_value(real_value);
          values[i] = real_value;
          //now time for keys   
          let mut real_keys = keys[i][3..].replace("("," ");
          real_keys = real_keys.replace(")"," ");
          real_keys = real_keys.trim().trim_end().to_string();
          real_keys = access_value(real_keys);
          keys[i] = real_keys;
         }
         else if slice == "AXC(" && key_slice == "ITR<"{
            //println!("We ARE IN SECOND IF");
            let mut real_value = values[i][3..].replace("("," ");
            real_value = real_value.replace(")"," ");
            real_value = real_value.trim().trim_end().to_string();
            //access the real value
            real_value = access_value(real_value);
            values[i] = real_value;
            //getting the keys from ITR eg. ITR<c<name
            let  real_key:Vec<&str> = keys[i][3..].split("<").collect();
            //check for integers.
            let index:usize = real_key[1].parse().unwrap();
            let key = real_key[2].to_string();
            let keys_value = itr_value(key, index,values[i].to_string(),relational_operator[i].to_string());
            keys[i] = keys_value;
            //after getting the real value just send the key,VECTOR INDEX AND DATA
         }
         else if slice == "ITR<" && key_slice == "AXC("{
            //println!("We ARE IN THIRD IF");
            let mut real_keys = keys[i][3..].replace("("," ");
            real_keys = real_keys.replace(")"," ");
            real_keys = real_keys.trim().trim_end().to_string();
            //access the real value
            real_keys = access_value(real_keys);
            keys[i] = real_keys;
            //getting the keys from ITR eg. ITR<c<name
            let  real_value:Vec<&str> = values[i][3..].split("<").collect();
            //check for integers.
            let index:usize = real_value[1].parse().unwrap();
            let value = real_value[2].to_string();
            let keys_value = itr_value(value, index,keys[i].to_string(),relational_operator[i].to_string());
            values[i] = keys_value;
         }
         else if slice == "AXC(" && key_slice!="AXC(" && key_slice!="ITR<"{
            //println!("We ARE IN FOURTH IF");

            let mut real_value = values[i][3..].replace("("," ");
            real_value = real_value.replace(")"," ");
            real_value = real_value.trim().trim_end().to_string();
            //access the real value
            real_value = access_value(real_value);
            values[i] = real_value;
         }
         else if key_slice == "AXC(" && slice!="AXC(" && slice!="ITR<"{
            //println!("We ARE IN FIFTH IF");

            let mut real_keys = keys[i][3..].replace("("," ");
            real_keys = real_keys.replace(")"," ");
            real_keys = real_keys.trim().trim_end().to_string();
            //access the real value
            real_keys = access_value(real_keys);
            keys[i] = real_keys;
             //PERFORM THE OPERATIONS 
         }
         else if slice == "ITR<" && key_slice !="ITR<" && key_slice !="AXC("{
            //println!("We ARE IN SIXTH IF");

            let  real_value:Vec<&str> = values[i][3..].split("<").collect();
            //check for integers.
            let index:usize = real_value[1].parse().unwrap();
            let value = real_value[2].to_string();
            let keys_value = itr_value(value, index,keys[i].to_string(),relational_operator[i].to_string());
            values[i] = keys_value;
            //PERFORM THE OPERATIONS
         }
         else if key_slice == "ITR<" && slice !="ITR<" && slice != "AXC("{
            //println!("We ARE IN SEVENTH IF");

            let  real_key:Vec<&str> = keys[i][3..].split("<").collect();
            //check for integers.
            let index:usize = real_key[1].parse().unwrap();
            let key = real_key[2].to_string();
            let keys_value = itr_value(key, index,values[i].to_string(),relational_operator[i].to_string());
            keys[i] = keys_value;
            //PERFORM THE OPERATIONS
         }
          else if slice == "ITR<" && key_slice == "ITR<"{
             print!("THIS PART IS EXPERIMENTAL AND NOT THOUGHT OF YET");
         }
         else{
             //println!("{} == ITR( && {} != ITR( && {} != AXC(",key_slice,slice,slice);
        // else if key_slice == "ITR(" && slice !="ITR(" && slice != "AXC("{

             //println!("FINALLY WE ARE HERE");
         }
        }
        else if values[i].len()>=4 && keys[i].len()<4{

            //start the original stuff from here
            let slice =  values[i][0..4].to_string();
         //let key_slice = keys[i][0..4].to_string();
         if slice == "AXC("{
            let mut real_value = values[i][3..].replace("("," ");
            real_value = real_value.replace(")"," ");
            real_value = real_value.trim().trim_end().to_string();
            //access the real value
            real_value = access_value(real_value);
            values[i] = real_value;
         }
         else if slice == "ITR<"{
             //getting the keys from ITR eg. ITR<c<name
            let  real_value:Vec<&str> = values[i][3..].split("<").collect();
            //check for integers.
            let index:usize = real_value[1].parse().unwrap();
            let key = real_value[2].to_string();
            let keys_value = itr_value(key, index,keys[i].to_string(),relational_operator[i].to_string());
            values[i] = keys_value;
         }
        }
        else if keys[i].len()>=4 && values[i].len()<4{

            //start the originals stuff
            let slice =  keys[i][0..4].to_string();
            //let key_slice = keys[i][0..4].to_string();
            if slice == "AXC("{
               let mut real_keys = keys[i][3..].replace("("," ");
               real_keys = real_keys.replace(")"," ");
               real_keys = real_keys.trim().trim_end().to_string();
               //access the real value
               real_keys = access_value(real_keys);
              // print!("real_key {:?}",real_keys);
               keys[i] = real_keys.trim().trim_end().to_string();
            }
            else if slice == "ITR<"{
                //getting the keys from ITR eg. ITR<c<name
               let  real_keys:Vec<&str> = keys[i][3..].split("<").collect();
               //check for integers.
               let index:usize = real_keys[1].parse().unwrap();
               let key = real_keys[2].to_string();
               let keys_value = itr_value(key, index,values[i].to_string(),relational_operator[i].to_string());
               keys[i] = keys_value;
            }

        }
        
    }

    //now we have real values
    /*
     Create VECTORS TO HOLD TRUE AND FALSE ONE COLLECTOR AND ANOTHER REAL
    */

    //println!("{:?}",values);
    let mut conditional_counter:i32 = -1;
    let mut i:usize = 0;
 
    if relational_operator.len() == 1{
        if relational_operator[0] == "eq"{
                if keys[i] == values[i]{
                   // let simple_str = generate_str(k);
                    data_to_send.push("true".to_string());
                }
        }
        else if relational_operator[0] == "neq"{
            if keys[i] != values[i]{
                // let simple_str = generate_str(k);
                 data_to_send.push("true".to_string());
             }
        }
        else{
            //throw an error.
        }
    }
    else if relational_operator.len() == 0{
        data_to_send.push("".to_string());
    }
    else{
        while i<relational_operator.len(){
            if i != relational_operator.len(){
                conditional_counter = conditional_counter+1;
        
            }   
        
               if conditional_counter == 0{
               if relational_operator[i] == "eq"{
                   if relational_operator[i+1]=="eq"{
                    if conditional_operator[conditional_counter as usize] == "AND"{
                        //loop will start from here for i to values.values.length
                           // println!("{:?} == {:?} AND {:?} == {:?}",data_set(k,keys[i]),values[i].to_string(),data_set(k,keys[i+1]),values[i+1].to_string());
                            if keys[i] == values[i] && keys[i+1] == values[i+1]{
                                    //insert data in data_to_send algorithm
                                    data_to_send.push("true".to_string());
                                //do something
                            }
                    }
                    else if conditional_operator[conditional_counter as usize] == "OR"{
                        
                        if keys[i] == values[i] || keys[i+1] == values[i+1]{
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                        }
                            //do something
                        
                    }
                    else{
                        //throw an error.
                    }
                   }
                   else if relational_operator[i+1]=="neq" {
                       if conditional_operator[conditional_counter as usize] == "AND"{
                        
                        if keys[i]== values[i] && keys[i+1] != values[i+1]{
                            if conditional_counter == 0{
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                            }
                            else{
       
                                data_collector.push("true".to_string());
                                //insert the data in data_to_collect algorithm
                            }
        
                        }
       
                       }
                       else if conditional_operator[conditional_counter as usize] == "OR"{
                        if keys[i] == values[i] || keys[i+1] != values[i+1]{
                                data_to_send.push("true".to_string());
                        }
                    
                       }
                   }
                   else{
                       //throw an error.
                   }
               }
               else if relational_operator[i] == "neq"{
                   if relational_operator[i+1] == "neq"{
                       if conditional_operator[conditional_counter as usize] == "AND"{
                   
                        if keys[i] != values[i] && keys[i+1] != values[i+1]{
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                        }
                       }
                       else if conditional_operator[conditional_counter as usize] == "OR"{
                        if keys[i] != values[i] || keys[i] != values[i+1]{
                           
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                        }
                    
                       }
                       else{
                           //throw an error.
                       }
                   }
                   else if relational_operator[i+1] == "eq"{
                       if conditional_operator[conditional_counter as usize] == "AND"{
                        if keys[i] != values[i] && keys[i+1] == values[i+1]{
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                        }
                       }
                       else if conditional_operator[conditional_counter as usize] == "OR"{
                        if keys[i] != values[i] || keys[i+1] == values[i+1]{
                                data_to_send.push("true".to_string());
                                //insert data in data_to_send algorithm
                        }
                    
                       }
                       else{
                           //throw an error Operators other than AND and OR.
                       }
                   }
                   else{
                       //throw an error Operators Other Than eq and neq.
                   }
               }
               else{
                   //throw an error
               }
               //how to increase the
               
               //i.checked_add(2).unwrap();
            }
            else {
           
                
                if relational_operator[i] == "eq"{
                            //tocheck
           
                            if keys[i] == values[i] {
                                
                                data_collector.push("true".to_string());
                              //  println!("{:?} to check",data_collector);
                            }
                        
                }
                else if relational_operator[i] == "neq"{
                        if keys[i] != values[i] {
                            data_collector.push("true".to_string());
                        }
                }
                else{
                    //throw an error.
                }
            }
               if conditional_counter>0{
                   //do this
        
                   if conditional_operator[conditional_counter as usize] == "AND"{
                       //call the set union 
                      
                       data_to_send = and_algo(data_to_send.clone(),data_collector.clone());
                       data_to_send = remove_duplicate(data_to_send);
                       data_collector = vec![];
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                      //merge both of the sets
                      data_to_send = or_algo(data_to_send.clone(),data_collector.clone());
                      data_to_send = remove_duplicate(data_to_send);
                      data_collector = vec![];
                   }
                   else{
                       println!("Throw An Error");
                   }
               }
            // conditional_counter = conditional_counter+1;
            if i == 0{
                i = i+2;
            }
            else{
                i = i+1;
        
            }
        
           }
    }
 
data_to_send
}

fn itr_value(key:String,index:usize,value:String,rop:String)->String{
    unsafe{

        let vectors = VECTORS[index].clone();

        let mut check = false;
        let mut string = "".to_string();
        for i in 0..vectors.len(){
            let  mut lets_create_key = vectors[i].replace("{"," ").to_string();
            lets_create_key = lets_create_key.replace("}"," ");
            lets_create_key = lets_create_key.trim().trim_end().to_string();
            let lets_create_key_:Vec<&str> = lets_create_key.split(",").collect();
            
            for j in lets_create_key_{
                let lets_split:Vec<&str> = j.trim().trim_end().split(":").collect();
                if lets_split[0].to_string() == key{

                    //check for the two possible operators.

                    
                    if value == lets_split[1].to_string(){
                        if rop == "eq"{
                            check = true;
                            string = lets_split[1].to_string();
                            break;
                        }
                        else if rop == "neq"{
                            check = true;
                            string = " ".to_string();
                            break;
                        }
                    }
                    
                }
            }
            if check{
                break;
            }
            //perform operations and send the data as per asked.
        }
        if check{
            string
        }
        else{
            " ".to_string()
        }
    }
}


fn update_one(query:String,index:usize)->Vec<usize>{
    //this has a some error like when we write name eq n1 AND name eq n2;
   //let query = String::from("Get From abc where name eq n1 AND age eq 2");

   
   let all_query:Vec<&str> = query.trim().trim_end().split(' ').collect();
   
   //let hello_data:(Vec<String>,Vec<String>) = get_single_vectors(data);
   //println!("keys {:?} and Values {:?}", data.0,data.1);
   let mut data = (vec![],vec![]);
   //let data:(Vec<String>,Vec<Vec<String>>) = data_model::send_model2();
   unsafe{
       data = OBJECTS[index].clone().1;
       KEYS = data.clone().0;
       VALUES = data.clone().1;
      
   }


   //Separate The Tokens Now.

   //we have to implement the IDEA FOR STRING QUOTE.

   //Separate All The Tokens

   
   
   let mut conditional_operator:Vec<&str> = vec![];
   let mut relational_operator:Vec<&str> = vec![];
   let mut keys:Vec<String> = vec![];
   let mut values:Vec<String> = vec![];
   let mut data_collector:Vec<usize> = vec![];
   let mut data_to_send:Vec<usize> = vec![];

   for i in 1..all_query.len(){
    //this loop is separating the tokens from the query;
    /*
    1.OR
    2.AND
    3.eq
    4.neq
     */
       match all_query[i]{
           "AND"=> conditional_operator.push(all_query[i]),
           "OR"=>conditional_operator.push(all_query[i]),
           "eq"=>
           {
            relational_operator.push(all_query[i]);
            keys.push(all_query[i-1].replace("!@#880"," "));
            values.push(all_query[i+1].replace("!@#880"," "));
           },
           "neq"=>
           {
            relational_operator.push(all_query[i]);
            keys.push(all_query[i-1].replace("!@#880"," "));
            values.push(all_query[i+1].replace("!@#880"," "));
           },
           _=>print!(" "),
       };
   }


   //CHECK FOR CONCATATION AND HASHOF WILL BE IMPLEMENTED HERE
   for i in 0..values.len(){
    if values[i].len()>7{
        if values[i][0..8].to_string() == "CONCATE("{
         let new_string = concate(values[i].clone());
         values[i] = new_string;
        }
        else if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
            let mut new_string = values[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             values[i] = new_string;
        }
        else if values[i][0..4].to_string() == "AFV("{
            values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
        }
        else if values[i].len()>9{
            if values[i][0..10].to_string() == "TIMESTAMP("{
                let new_value = af::time_stamp();
                values[i] = new_value;
            }
        }
    }
    else if values[i].len()>4{
        if values[i][0..5].to_string() == "UID()"{
            let new_string = af::generate_uid();
            values[i] = new_string;
        }
        else if values[i][0..4].to_string() == "AFV("{
            values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
        }
        else if values[i].len()>5{
            if values[i][0..6].to_string() == "INPUT(" && values[i].to_string().chars().nth(values[i].len()-1) == Some(')'){
                let mut new_string = values[i].replace("INPUT(","");
                 new_string = new_string.replace(")","");
                 new_string = give_value(new_string);
                 values[i] = new_string;
            }
        }
    }
    
    if keys[i].len()>7{
        if keys[i][0..8].to_string() == "CONCATE("{
            let new_key = concate(keys[i].clone());
            keys[i] = new_key;
        }
        else  if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
            let mut new_string = keys[i].replace("INPUT(","");
             new_string = new_string.replace(")","");
             new_string = give_value(new_string);
             keys[i] = new_string;
        }
        else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(values[i][4..keys[i].len()-1].to_string()).1;
        }
        else if keys[i].len()>9{
            if keys[i][0..10].to_string() == "TIMESTAMP("{
                let new_key = af::time_stamp();
                keys[i] = new_key;
            }
        }
    }
    else if keys[i].len()>4{
        if keys[i][0..5].to_string() == "UID()"{
            let new_string = af::generate_uid();
            keys[i] = new_string;
        }
        else if keys[i][0..4].to_string() == "AFV("{
            keys[i] = afv(values[i][4..keys[i].len()-1].to_string()).1;
        }
        else if keys[i].len()>5{
            if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                let mut new_string = keys[i].replace("INPUT(","");
                 new_string = new_string.replace(")","");
                 new_string = give_value(new_string);
                 keys[i] = new_string;
            }
        }
    }
}
   //Access TO The Values

  

   for i in 0..values.len(){
       if values[i].len()>4{
        let slice =  values[i][0..4].to_string();
        if slice == "AXC("{
         let mut real_value = values[i][3..].replace("("," ");
         real_value = real_value.replace(")"," ");
         real_value = real_value.trim().trim_end().to_string();
         //access the real value
         real_value = access_value(real_value);
         values[i] = real_value;
 
        }
       }
       
   }
   let   key_check = cmp_for_keys(keys.clone());
   let mut key_index = vec![];
   if key_check.0{
       key_index = key_check.1;
   }
   else{
    println!("error column not matched");
    return vec![]
   }

   let mut conditional_counter:i32 = -1;
   let mut i:usize = 0;

    if relational_operator.len() == 1{
    if relational_operator[0] == "eq"{
        for k in 0..data.clone().1[0].len(){
            if data.clone().1[key_index[i]][k] == values[i].clone(){
               // let simple_str = generate_str(k);
                data_to_send.push(k);
            }
        }
    }
    else if relational_operator[0] == "neq"{
        for k in 0..data.clone().1[0].len(){
            if data.clone().1[key_index[i]][k] != values[i].clone(){
                //let simple_str = generate_str(k);
                data_to_send.push(k);
            }
        }
    }
    else{
        //throw an error.
    }
}
else if relational_operator.len() == 0{

}
else{

    while i<relational_operator.len(){
        if i != relational_operator.len(){
            conditional_counter = conditional_counter+1;
    
        }   
    
           if conditional_counter == 0{
           if relational_operator[i] == "eq"{
               if relational_operator[i+1]=="eq"{
                if conditional_operator[conditional_counter as usize] == "AND"{
                    //loop will start from here for i to values.values.length
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() && data.clone().1[key_index[i+1]][k] == values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                }
                else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() || data.clone().1[key_index[i+1]][k] == values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                }
                else{
                    //throw an error.
                }
               }
               else if relational_operator[i+1]=="neq" {
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() && data.clone().1[key_index[i+1]][k] != values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] == values[i].clone() || data.clone().1[key_index[i+1]][k] != values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                   }
               }
               else{
                   //throw an error.
               }
           }
           else if relational_operator[i] == "neq"{
               if relational_operator[i+1] == "neq"{
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() && data.clone().1[key_index[i+1]][k] != values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
    
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() || data.clone().1[key_index[i+1]][k] != values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                   }
                   else{
                       //throw an error.
                   }
               }
               else if relational_operator[i+1] == "eq"{
                   if conditional_operator[conditional_counter as usize] == "AND"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() && data.clone().1[key_index[i+1]][k] == values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                   }
                   else if conditional_operator[conditional_counter as usize] == "OR"{
                    for k in 0..data.clone().1[0].len(){
                        if data.clone().1[key_index[i]][k] != values[i].clone() || data.clone().1[key_index[i+1]][k] == values[i+1]{
                            //let simple_str = generate_str(k);
                            data_to_send.push(k);
                        }
                    }
                   }
                   else{
                       //throw an error Operators other than AND and OR.
                   }
               }
               else{
                   //throw an error Operators Other Than eq and neq.
               }
           }
           else{
               //throw an error
           }
           //how to increase the
           
           //i.checked_add(2).unwrap();
        }
        else {
       
            
            if relational_operator[i] == "eq"{
                for k in 0..data.clone().1[0].len(){
                    if data.clone().1[key_index[i]][k] == values[i].clone(){
                        //let simple_str = generate_str(k);
                        data_to_send.push(k);
                    }
                }
            }
            else if relational_operator[i] == "neq"{
                for k in 0..data.clone().1[0].len(){
                    if data.clone().1[key_index[i]][k] != values[i].clone(){
                        //let simple_str = generate_str(k);
                        data_to_send.push(k);
                    }
                }
            }
            else{
                //throw an error.
            }
        }
           if conditional_counter>0{
               //do this
    
               if conditional_operator[conditional_counter as usize] == "AND"{
                   //call the set union 
                  
                   data_to_send = and_algo_usize(data_to_send.clone(),data_collector.clone());
                   data_to_send = remove_duplicate_usize(data_to_send);
                   data_collector = vec![];
               }
               else if conditional_operator[conditional_counter as usize] == "OR"{
                  //merge both of the sets
                  data_to_send = or_algo_usize(data_to_send.clone(),data_collector.clone());
                  data_to_send = remove_duplicate_usize(data_to_send);
                  data_collector = vec![];
               }
               else{
                   println!("Throw An Error");
               }
           }
        // conditional_counter = conditional_counter+1;
        if i == 0{
            i = i+2;
        }
        else{
            i = i+1;
    
        }
    
       }
    
}
   //we will create the conditions chain for the rest of the programs
  // println!("{:?}",data_to_send);
   data_to_send
}

fn delete(index:usize,index1:usize){
    unsafe{
        let mut data = OBJECTS[index].clone().1;
        for i in 0..data.0.len(){
              data.1[i].remove(index1);
        }
        //VALUES.remove(index);
        OBJECTS[index].1 = data;
    }
}

fn update_second(query:String,index:usize,index1:usize){
    //UPDATE ABC WHERE NAME eq AMAN AND AGE eq 12 ( SET {name:,age:,value:,});
    /*
    1.CHECK COLUMN PRESENCE ONCE BEFORE INSERTING THE VALUES*/
    
    let mut data_at_index = (vec![],vec![]);
    unsafe{
        data_at_index = OBJECTS[index1].clone().1;
    }

    let mut  clearify_query = query.trim().trim_end().to_string();
    if clearify_query.chars().nth(0) == Some('{') && clearify_query.chars().nth(clearify_query.len()-1) == Some('}'){
       //remove the '{' '}'

       clearify_query = clearify_query.replace("{"," ");
       clearify_query = clearify_query.replace("}"," ");
       clearify_query = clearify_query.trim().trim_end().to_string();
       //After Parsing The Query We Needed To Indexed It.

       let query_vec:Vec<&str> = clearify_query.split(',').collect();
       let mut values:Vec<String> = vec![];
       let mut keys:Vec<String> = vec![];

       for i in 0..query_vec.len(){
           let sp:Vec<&str> = query_vec[i].split(":").collect();
           values.push(sp[1].to_string());
           keys.push(sp[0].to_string());
       }

       //Now We Have Keys And Values Lets PARSE KEYS AND VALUES INTO THEIR NATIVE PART
       let mut real_value = String::from("");
       for i in 0..values.len(){
        if values[i].len()>4{
         let slice =  values[i][0..4].to_string();
         if slice == "AXC("{
           real_value = values[i][3..].replace("("," ");
          real_value = real_value.replace(")"," ");
          real_value = real_value.trim().trim_end().to_string();
          //access the real value
          real_value = access_value(real_value);
          values[i] = real_value;
         }
         else if values[i][0..5].to_string() == "UID()"{
             //Check The Length Of The Programe For The Rest Of The Functions And It Is The Necessary Evil
             values[i] = af::generate_uid();
         }
         else if values[i][0..4].to_string() == "AFV("{
                values[i] = afv(values[i][4..values[i].len()-1].to_string()).1;
         }
         //add the length of the programes
         if values[i].len()>5{
             if values[i][0..6].to_string() == "INPUT("{
                // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                      real_value = values[i].replace("INPUT(","");
                      real_value = real_value.replace(")","");
                      real_value = give_value(real_value);
                      values[i] = real_value;
             }
             else if values[i].len()>7{

                 //for the concate values
                 if values[i][0..8].to_string() == "CONCATE("{
                     //for concate 
                     real_value = concate(values[i].clone());
                     values[i] = real_value;
                 }
                 //we have to call the timestamp here as well
                 else if values[i].len()>9{
                    //call the timestamp programe here
                    if values[i][0..10].to_string() == "TIMESTAMP("{
                        values[i] = af::time_stamp();
                    } 
                }
             }
             
         }
        }
       //let mut returned_string = generate_str(index);
       //returned_string = returned_string.replace("{"," ");
       //returned_string = returned_string.replace("}"," ");
       //returned_string = returned_string.trim().trim_end().to_string();
       //let returned_vec:Vec<&str> = returned_string.split(",").collect();
       for i in 0..data_at_index.0.len(){
           
          //let mut splitted_value:Vec<&str> = returned_vec[i].split(":").collect();
           //match the key
           //println!("older values {:?}",splitted_value[1]);
           for j in 0..keys.len(){
               if data_at_index.0[i] == keys[j]{
                   unsafe{
                       data_at_index.1[i][index1] = values[j].clone();
                       OBJECTS[index].1 = data_at_index.clone();
                   }
               }
           }
           //println!("newer values {:?}",splitted_value[1]);
       }     
    }
    //LETS GET DATA FRM A AND B.

    }

    //RETURN VALUE IN THE FUTURE.

}

fn store_update(store_query:&str){
    let  data_to = &store_query.replace("([","").to_string();
      let  data_to = data_to.replace("])","").to_string();
      //let  data_to = data_to.replace(")"," ").to_string();
      //let  data_to = data_to.replace("]"," ").to_string();
      let data_to = data_to.trim().trim().to_string().replace("[","").replace("]","");
      let data_to = data_to.trim_end();


        let values:Vec<&str> =  data_to.split("|").collect();
        //println!("{:?}",values);
        for i in values{
            let check_i = i.replace("{"," ");
            let check_i = check_i.replace("}"," ");
            let  check_i = check_i.trim();
            let  check_i = check_i.trim_end();

                let mut key_value:Vec<&str> = check_i.split(':').collect();
                //GET THE KEY INDEX AND SHOW THEIR VALUES.

                let key_index = get_key_index(key_value[0].to_string());
                if key_index.0{
                    let split_check:Vec<&str> = key_value[1].split('<').collect();
                    let mut new_value = String::from("");

                    if key_value[1].len()>7{
                        if key_value[1][0..8].to_string() == "CONCATE("{
                            new_value = concate(key_value[1].to_string());
                           key_value[1] = &new_value;
                       }
                       else if key_value[1][0..4].to_string() == "AXC("{
                        new_value = key_value[1].replace("AXC(","");
                        new_value = new_value.replace(")","");
                        new_value = access_value(new_value.trim().trim_end().to_string());
                        key_value[1] = &new_value;
                       }
                       else if key_value[1][0..5].to_string() == "UID()"{
                        new_value = af::generate_uid();
                        key_value[1] = &new_value;
                    }
                    else if key_value[1][0..4].to_string() == "AFV("{
                        new_value = afv(key_value[1][4..key_value[1].len()-1].to_string()).1;
                        key_value[1] = &new_value;
                    }
                    else if key_value[1][0..6].to_string() == "INPUT("{
                       
                           // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                                 new_value = key_value[1].replace("INPUT(","");
                                 new_value = new_value.replace(")","");
                                 new_value = give_value(new_value);
                                 key_value[1] = &new_value;
                    }
                       else if key_value[1].len()>9{
                           if key_value[1][0..10].to_string() == "TIMESTAMP("{
                               new_value = af::time_stamp();
                               key_value[1] = &new_value;
                           }
                       }
                    }
                    else if key_value[1].len()>3{
                        if key_value[1][0..4].to_string() == "AXC("{
                            //we have to create a variable to access the values;
                            new_value = key_value[1].replace("AXC(","");
                            new_value = new_value.replace(")","");
                            new_value = access_value(new_value.trim().trim_end().to_string());
                            key_value[1] = &new_value;
                        }
                        else if key_value[1].len()>4{
                            if key_value[1][0..5].to_string() == "UID()"{
                                new_value = af::generate_uid();
                                key_value[1] = &new_value;
                            }
                            else if key_value[1][0..4].to_string() == "AFV("{
                                new_value = afv(key_value[1][4..key_value[1].len()-1].to_string()).1;
                                key_value[1] = &new_value;
                            }
                            else if key_value[1].len()>5{
                                 if key_value[1][0..6].to_string() == "INPUT("{
                       
                                    // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                                          new_value = key_value[1].replace("INPUT(","");
                                          new_value = new_value.replace(")","");
                                          new_value = give_value(new_value);
                                          key_value[1] = &new_value;
                             }
                            }
                        }
                       
                    }

                
                
                    if split_check.len()>1{
                        //println!("{:?} splitted value",split_check);
                        if split_check[1] == "c"{
                             
                            unsafe{
    
                            let current = VECTORS.len()-1 as usize;
                            let get = to_get(VECTORS[current].clone(),split_check[2]);
                            LOCATION_STORAGE[key_index.1] = get;
                            //LOCATION_STORAGE.push(get);
                            //LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                            }
                            }
                        else{
                            println!("IT IS NOT CURRENT VALUE");
                            unsafe{
                                let current = split_check[1].to_string().parse::<i32>().unwrap() as usize;
                                if current<VECTORS.len(){
                                let get = to_get(VECTORS[current].clone(),split_check[2]);
                                LOCATION_STORAGE[key_index.1] = get;
                                //LOCATION_STORAGE.push(get);
                                //LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                                }
                            else{
                                //throw error;
                            } 
                           }
                            
                        }
                    }
                    else{
                        //normal value
                        let put = vec![key_value[1].to_string()];
                        unsafe{
                            LOCATION_STORAGE[key_index.1] = put;

                            //LOCATION.push((key_value[0].to_string(),LOCATION_STORAGE.len()-1));
                        }
                    }
                }
                else{
                    //KEY NOT FOUND;
                    println!("KEY NOT FOUND ERROR");
                }     
            
        }   
}

fn get_key_index(key:String)->(bool,usize){
   unsafe{
       let mut check = false;
       let mut index:usize = 0;
       for i in 0..LOCATION.len(){
           if LOCATION[i].0 == key{
               check = true;
               index = i;
               break;
           }
       }
       if check{
           (check,index)
       }
       else{
           (check,index)
       }
   }
}

fn concate(values:String)->String{
    unsafe{
    let mut vl = values.replace("CONCATE(","").trim().trim_end().to_string();
    vl.replace_range(vl.len()-1..,"");
    let vl_vector:Vec<&str> = vl.split(",").collect();
    let mut new_string = String::from("");
    for i in vl_vector{
        if i.len()>3{
            if i[0..4].to_string() == "AXC("{
                new_string.push_str(&access_value(i.replace("AXC(","").to_string().replace(")","").trim().trim_end().to_string()));
            }
            else if i[0..4].to_string() == "ITR<"{
                let split_check:Vec<&str> = i.split("<").collect();
                
                    let current = split_check[1].to_string().parse::<i32>().unwrap() as usize;
                    if current<VECTORS.len(){
                    let get = to_get(VECTORS[current].clone(),split_check[2]);
                    new_string.push_str(&get[get.len()-1]);
                    }
                    else{
                        println!("DATA NOT FOUND");
                    }
                }
                else if i.len()>4{
                    if i[0..5].to_string() == "UID()"{
                        new_string.push_str(&af::generate_uid());
                    }
                    else if i[0..4].to_string() == "AFV("{
                        new_string.push_str(&afv(i[4..i.len()-1].to_string()).1);
                    }
                else if i.len()>5{
                    if i[0..6].to_string() == "INPUT("{
                            // if keys[i][0..6].to_string() == "INPUT(" && keys[i].to_string().chars().nth(keys[i].len()-1) == Some(')'){
                                 let mut new_value = i.replace("INPUT(","");
                                  new_value = new_value.replace(")","");
                                  new_value = give_value(new_value);
                                  //key_value[1] = &new_value;
                                  new_string.push_str(&new_value);
                     }
                     else{
                      new_string.push_str(i);
                     }
                    }
                }
                else if i.len() >9{
                    if i[0..10].to_string() == "TIMESTAMP("{
                        new_string.push_str(&af::time_stamp());
                    }
                  new_string.push_str(i);
                }
                else{
                new_string.push_str(i);
                }  
        }    
         //new_string.push_str(&access_value(i.replace("AXC(","").to_string().replace(")","").trim().trim_end().to_string()));
        else{
            new_string.push_str(i);
        }
        //before pushing check for the AXC AND ITR<
    }
new_string
}
}

fn return_(i:String)->(String,Vec<Vec<String>>){
    unsafe{
        let return_indicator:Vec<&str> = i.split(">>").collect();
        let mut key_to_send = "".to_string();
        let mut vector_to_send:Vec<Vec<String>> = vec![];
        if return_indicator.len() == 2{
            let mut return_stat = return_indicator[1].to_string();
            return_stat = return_stat.trim().trim_end().to_string();
            if return_stat[0..7].to_string() == "RETURN("{
                return_stat = return_stat.replace("RETURN(","");
                return_stat = return_stat.replace(")","");
                return_stat = return_stat.trim().trim_end().to_string();
                let split_for_return:Vec<&str> = return_stat.split(",").collect();
                //check for the end values of the splitted arguments
                if split_for_return.len() == 2{
                    if split_for_return[0].len()>3{
                     //   println!("{:?} splitter",split_for_return[0][0..4].to_string());
                        if split_for_return[0][0..4].to_string() == "AXC("{
                            //ACCESS THE KEY AND THEN GO FOR THE VECTOR PART
                            let mut k = split_for_return[0].replace("AXC(","");
                             k = k.replace(")","");
                             //now I have the key
                             k = access_value(k);
                             key_to_send = k;
                        } 
                        else if split_for_return[0][0..4].to_string() == "ITR<"{
                            
                                let itr_splitter:Vec<&str> = split_for_return[0].split("<").collect();
                                if itr_splitter.len() == 3{
                                    let index:usize = itr_splitter[1].parse().unwrap();
                                    if VECTORS.len()>index{
                                        let k = to_get(VECTORS[index].clone(),itr_splitter[2]);
                                        key_to_send = k[k.len()-1].to_string();
                                    }
                                    else{
                                        println!("Array Out Of Index");
                                    }
                                } 
                                else{
                                    println!("ITR SPLITTER IS NOT WORKING");
                                }
                        }
                        else if split_for_return[0].len()>4{
                            if split_for_return[0][0..5].to_string() == "UID()"{
                                key_to_send = af::generate_uid();
                            }
                            else if split_for_return[0][0..4].to_string() == "AFV("{
                                key_to_send = afv(split_for_return[0][4..split_for_return[0].len()-1].to_string()).1;
                            //    key_value[1] = &new_value;
                            }
                            else if split_for_return[0][0..6].to_string() == "INPUT("{
                                key_to_send = split_for_return[0].replace("INPUT(","");
                                key_to_send = key_to_send.replace(")","");
                                key_to_send = give_value(key_to_send);
                            }
                            else{
                                key_to_send = split_for_return[0].to_string();
                            }
                        }
                        else if split_for_return[0].len()>9{
                            if split_for_return[0][0..10].to_string() == "TIMESTAMP("{
                                key_to_send = af::time_stamp();
                            }
                            else{
                                key_to_send = split_for_return[0].to_string();
                            }
                        }
                        else{
                            key_to_send = split_for_return[0].to_string();
                        }
                    }
                    //check for AXC AND OTHER DATAS
                    else{
                        key_to_send = split_for_return[0].to_string();
                    }
                    //Accessing the second value and writting the conditions for it.
                    let mut vector_sender = split_for_return[1].trim().trim_end().to_string();
                   // println!("{:?} and {:?}",vector_sender.chars().nth(0),vector_sender.chars().nth(vector_sender.len()-1));
                    if vector_sender.chars().nth(0) == Some('<') && vector_sender.chars().nth(vector_sender.len()-1)== Some('>'){
                        vector_sender = vector_sender.replace("<","");
                        vector_sender = vector_sender.replace(">","");
                        vector_sender = vector_sender.trim().trim_end().to_string();
                        let  vec_collector:Vec<&str> = vector_sender.split("|").collect();
                     //   println!("{:?}",vec_collector);
                        if vec_collector.len()>0 && vec_collector[0]!=""{
                            for i in vec_collector{
                                let index:usize = i.parse().unwrap();
                                if index<VECTORS.len(){
                                    vector_to_send.push(VECTORS[index].clone());
                                }
                            }
                        }
                    }
                    else{
                        //throw error
                        println!("Possibly An Error Inside The Programe");
                    }
                }
                else{
                    println!("RETURN VALUE MUST CONSIST OF 2 Statements Sparated by ','");
                }
            }
        }
        (key_to_send,vector_to_send)     
    }
}



fn rfv_(i:String)->(String,String,Vec<Vec<String>>){
    unsafe{
        let return_indicator:Vec<&str> = i.split(">>").collect();
        let mut key_to_send = "".to_string();
        let mut vector_to_send:Vec<Vec<String>> = vec![];
        let mut name_to_send:String = "".to_string();
        if return_indicator.len() == 2{
            let mut return_stat = return_indicator[1].to_string();
            return_stat = return_stat.trim().trim_end().to_string();
            if return_stat[0..4].to_string() == "RFV("{
                return_stat = return_stat.replace("RFV(","");
                return_stat = return_stat.replace(")","");
                return_stat = return_stat.trim().trim_end().to_string();
                let split_for_return:Vec<&str> = return_stat.split(",").collect();
                //check for the end values of the splitted arguments
                if split_for_return.len() == 3{
                    name_to_send = split_for_return[0].to_string();
                    if split_for_return[1].len()>3{
                     //   println!("{:?} splitter",split_for_return[0][0..4].to_string());
                        if split_for_return[1][0..4].to_string() == "AXC("{
                            //ACCESS THE KEY AND THEN GO FOR THE VECTOR PART
                            let mut k = split_for_return[1].replace("AXC(","");
                             k = k.replace(")","");
                             //now I have the key
                             k = access_value(k);
                             key_to_send = k;
                        } 
                        else if split_for_return[1][0..4].to_string() == "ITR<"{
                            
                                let itr_splitter:Vec<&str> = split_for_return[1].split("<").collect();
                                if itr_splitter.len() == 3{
                                    let index:usize = itr_splitter[1].parse().unwrap();
                                    if VECTORS.len()>index{
                                        let k = to_get(VECTORS[index].clone(),itr_splitter[2]);
                                        key_to_send = k[k.len()-1].to_string();
                                    }
                                    else{
                                        println!("Array Out Of Index");
                                    }
                                } 
                                else{
                                    println!("ITR SPLITTER IS NOT WORKING");
                                }
                        }
                        else if split_for_return[1].len()>4{
                            if split_for_return[1][0..5].to_string() == "UID()"{
                                key_to_send = af::generate_uid();
                            }
                            else if split_for_return[0][0..4].to_string() == "AFV("{
                                key_to_send = afv(split_for_return[0][4..split_for_return[0].len()-1].to_string()).1;
                            //    key_value[1] = &new_value;
                            }
                            else if split_for_return[1][0..6].to_string() == "INPUT("{
                                key_to_send = split_for_return[0].replace("INPUT(","");
                                key_to_send = key_to_send.replace(")","");
                                key_to_send = give_value(key_to_send);
                            }
                            else{
                                key_to_send = split_for_return[1].to_string();
                            }
                        }
                        else if split_for_return[1].len()>9{
                            if split_for_return[1][0..10].to_string() == "TIMESTAMP("{
                                key_to_send = af::time_stamp();
                            }
                            else{
                                key_to_send = split_for_return[1].to_string();
                            }
                        }
                        else{
                            key_to_send = split_for_return[1].to_string();
                        }
                    }
                    //check for AXC AND OTHER DATAS
                    else{
                        key_to_send = split_for_return[1].to_string();
                    }
                    //Accessing the second value and writting the conditions for it.
                    let mut vector_sender = split_for_return[2].trim().trim_end().to_string();
                   // println!("{:?} and {:?}",vector_sender.chars().nth(0),vector_sender.chars().nth(vector_sender.len()-1));
                    if vector_sender.chars().nth(0) == Some('<') && vector_sender.chars().nth(vector_sender.len()-1)== Some('>'){
                        vector_sender = vector_sender.replace("<","");
                        vector_sender = vector_sender.replace(">","");
                        vector_sender = vector_sender.trim().trim_end().to_string();
                        let  vec_collector:Vec<&str> = vector_sender.split("|").collect();
                     //   println!("{:?}",vec_collector);
                        if vec_collector.len()>0 && vec_collector[0]!=""{
                            for i in vec_collector{
                                let index:usize = i.parse().unwrap();
                                if index<VECTORS.len(){
                                    vector_to_send.push(VECTORS[index].clone());
                                }
                            }
                        }
                    }
                    else{
                        //throw error
                        println!("Possibly An Error Inside The Programe");
                    }
                }
                else{
                    println!("RETURN VALUE MUST CONSIST OF 2 Statements Sparated by ','");
                }
            }
        }
        (name_to_send,key_to_send,vector_to_send)     
    }
}








fn cmp_rfv(i:String)->(String,String,Vec<Vec<String>>){
    let mut string_to_send = "".to_string();
    let mut vec_to_send:Vec<Vec<String>> = vec![];
    let mut name_to_send = "".to_string();
    let mut i_string_to_send = i.replace("RFV(","");
    i_string_to_send = i_string_to_send.replace(")","");
    i_string_to_send = i_string_to_send.trim().trim_end().to_string();
    let splitter:Vec<&str> = i_string_to_send.split(",").collect();
    if splitter.len() == 3{
        name_to_send = splitter[0].to_string();
        if splitter[1].len()>3{
            if splitter[1][0..4].to_string() == "AXC("{
                //ACCESS THE KEY AND THEN GO FOR THE VECTOR PART
                let mut k = splitter[0].replace("AXC(","");
                 k = k.replace(")","");
                 //now I have the key
                 k = access_value(k);
                 string_to_send = k;
            } 
            else if splitter[1][0..4].to_string() == "ITR<"{
                
                    let itr_splitter:Vec<&str> = splitter[1].split("<").collect();
                    unsafe{
                    if itr_splitter.len() == 3{
                        let index:usize = itr_splitter[1].parse().unwrap();
                        if VECTORS.len()>index{
                            let k = to_get(VECTORS[index].clone(),itr_splitter[2]);
                            string_to_send = k[k.len()-1].to_string();
                        }
                        else{
                            println!("Array Out Of Index");
                        }
                    } 
                    else{
                        println!("ITR SPLITTER IS NOT WORKING");
                    }
                }
                
            }
            else{
                string_to_send = splitter[1].to_string();
            }
        }
        else if splitter[1].len()>9{
            if splitter[1][0..10].to_string() == "TIMESTAMP("{
                string_to_send = af::time_stamp();
            }
             else if splitter[1][0..6].to_string() == "INPUT("{
                string_to_send = splitter[1].replace("INPUT(","");
                string_to_send = string_to_send.replace(")","");
                string_to_send = give_value(string_to_send);
            }
            else{
                string_to_send = splitter[1].to_string();
            }
        }
        else if splitter[1].len()>4{
            if splitter[1][0..5].to_string() == "UID()"{
                string_to_send = af::generate_uid();
            }
            else if splitter[0][0..4].to_string() == "AFV("{
                string_to_send = afv(splitter[0][4..splitter[0].len()-1].to_string()).1;
            //    key_value[1] = &new_value;
            }
            else if splitter[1][0..6].to_string() == "INPUT("{
                string_to_send = splitter[0].replace("INPUT(","");
                string_to_send = string_to_send.replace(")","");
                string_to_send = give_value(string_to_send);
            }
            else{
                string_to_send = splitter[1].to_string();
            }
        }
        else{
            string_to_send = splitter[0].to_string();
        }

        let mut vector_sender = splitter[2].trim().trim_end().to_string();
        unsafe{
           // println!("{:?}",splitter);
            //println!("{:?} and {:?}",vector_sender.chars().nth(0),vector_sender.chars().nth(vector_sender.len()-1));
        if vector_sender.chars().nth(0) == Some('<') && vector_sender.chars().nth(vector_sender.len()-1)== Some('>'){
            vector_sender = vector_sender.replace("<","");
            vector_sender = vector_sender.replace(">","");
            vector_sender = vector_sender.trim().trim_end().to_string();
            let  vec_collector:Vec<&str> = vector_sender.split("|").collect();
            if vec_collector.len()>0 && vec_collector[0]!=""{
            for i in vec_collector{
                let index:usize = i.parse().unwrap();
                if index<VECTORS.len(){
                    vec_to_send.push(VECTORS[index].clone());
                }
            }
        }
        }
        else{
            //throw error
            println!("Possibly An Error Inside The Programe");
        }
    }
    }
    else{
        panic!("RFV needs three arguments RFV(name,message,vec)");
    }

    (name_to_send,string_to_send,vec_to_send)

}

fn cmp_return(i:String)->(String,Vec<Vec<String>>){
    //i would be like RETURN(AXC(key)), ITR<Index<key,normal message;
    let mut string_to_send = "".to_string();
    let mut vec_to_send:Vec<Vec<String>> = vec![];
    let mut i_string_to_send = i.replace("RETURN(","");
    i_string_to_send = i_string_to_send.replace(")","");
    i_string_to_send = i_string_to_send.trim().trim_end().to_string();
    let splitter:Vec<&str> = i_string_to_send.split(",").collect();
    if splitter.len() == 2{
            //check for AXC AND OTHER DATAS
            if splitter[0].len()>3{
                if splitter[0][0..4].to_string() == "AXC("{
                    //ACCESS THE KEY AND THEN GO FOR THE VECTOR PART
                    let mut k = splitter[0].replace("AXC(","");
                     k = k.replace(")","");
                     //now I have the key
                     k = access_value(k);
                     string_to_send = k;
                } 
                else if splitter[0][0..4].to_string() == "ITR<"{
                    
                        let itr_splitter:Vec<&str> = splitter[0].split("<").collect();
                        unsafe{
                        if itr_splitter.len() == 3{
                            let index:usize = itr_splitter[1].parse().unwrap();
                            if VECTORS.len()>index{
                                let k = to_get(VECTORS[index].clone(),itr_splitter[2]);
                                string_to_send = k[k.len()-1].to_string();
                            }
                            else{
                                println!("Array Out Of Index");
                            }
                        } 
                        else{
                            println!("ITR SPLITTER IS NOT WORKING");
                        }
                    }
                    
                }
                else{
                    string_to_send = splitter[0].to_string();
                }
            }
            else if splitter[0].len()>9{
                if splitter[0][0..10].to_string() == "TIMESTAMP("{
                    string_to_send = af::time_stamp();
                }
                 else if splitter[0][0..6].to_string() == "INPUT("{
                    string_to_send = splitter[0].replace("INPUT(","");
                    string_to_send = string_to_send.replace(")","");
                    string_to_send = give_value(string_to_send);
                }
                else{
                    string_to_send = splitter[0].to_string();
                }
            }
            else if splitter[0].len()>4{
                if splitter[0][0..5].to_string() == "UID()"{
                    string_to_send = af::generate_uid();
                }
                else if splitter[0][0..4].to_string() == "AFV("{
                    string_to_send = afv(splitter[0][4..splitter[0].len()-1].to_string()).1;
                //    key_value[1] = &new_value;
                }
                else if splitter[0][0..6].to_string() == "INPUT("{
                    string_to_send = splitter[0].replace("INPUT(","");
                    string_to_send = string_to_send.replace(")","");
                    string_to_send = give_value(string_to_send);
                }
                else{
                    string_to_send = splitter[0].to_string();
                }
            }
            else{
                string_to_send = splitter[0].to_string();
            }
            //Accessing the second value and writting the conditions for it.
            let mut vector_sender = splitter[1].trim().trim_end().to_string();
            unsafe{
                //println!("{:?}",splitter);
               // println!("{:?} and {:?}",vector_sender.chars().nth(0),vector_sender.chars().nth(vector_sender.len()-1));
            if vector_sender.chars().nth(0) == Some('<') && vector_sender.chars().nth(vector_sender.len()-1)== Some('>'){
                vector_sender = vector_sender.replace("<","");
                vector_sender = vector_sender.replace(">","");
                vector_sender = vector_sender.trim().trim_end().to_string();
                let  vec_collector:Vec<&str> = vector_sender.split("|").collect();
                if vec_collector.len()>0 && vec_collector[0]!=""{
                for i in vec_collector{
                    let index:usize = i.parse().unwrap();
                    if index<VECTORS.len(){
                        vec_to_send.push(VECTORS[index].clone());
                    }
                }
            }
            }
            else{
                //throw error
                println!("Possibly An Error Inside The Programe");
            
            }
        }
    //now we have the key and we have to go ahead
}
(string_to_send,vec_to_send)
}

//give values from INPUT_VALUES

fn give_value(key:String)->String{
    unsafe{
        let mut string_to_send = String::from("");
        for i in INPUT_STORAGE.clone(){
            if i.0 == key{
                string_to_send = i.1;
                break;
            }
        }
        string_to_send
    }
}

fn function_executer(querys_:String){
    let querys_ = querys_.replace("\n","").trim().trim_end().to_string();
    let l_data_querys:Vec<&str> = querys_.trim().trim_end().split("<<-").collect();
    //call the basic methods

    for i in l_data_querys.clone(){
        let all_query:Vec<&str> = i.split(" ").collect();
        //now we have what we wanted
        if all_query[0] == "FUN"{
            let checker:Vec<&str> = i.split("->>").collect();
            if checker.len() == 2{
                let names = all_query[1].to_string();
                 let mut instructions = checker[1].to_string();
                 instructions = instructions.trim().trim_end().to_string();
                // if instructions.chars().nth(0) == Some('{') && instructions.chars().nth(instructions.len()-1) == Some('}'){
                  //   instructions.replace_range(..0,"");
                    // instructions.replace_range(instructions.len()-1..,"");
                     //now we have pure instructions
                     //create funtions for check
                     unsafe{
                      let freq = af::count_freq_tuple(FUNCTION.clone(),names.clone());
                      if freq ==0{
                          //now insert the data
                          FUNCTION.push((names,instructions));
                      }
                      else{
                          panic!("Function With Name {} Already Exists",all_query[1]);
                      }
                     }
               //  }
                // else{
                  //   panic!("Check For '{' and '}' tokens in Function Prototype");
                 //}
            }
            else{
                panic!("Check The Function Prototype");
            }
        }

    }
    instruction_executer(l_data_querys[l_data_querys.len()-1].to_string());
}

fn instruction_executer(querys_:String){

    let l_data_querys:Vec<&str> = querys_.trim().trim_end().split(";").collect();
    let mut  querys:Vec<String> = vec![];
    for l in l_data_querys{
        let mut new_query = String::from("");
        let mut start = 0;
        let mut end = 0;
        let mut a = 0;
       // let mut bait_value = String::from("");
 
       let mut query = l.replace('\n'," ").trim().trim_end().to_string().replace("  "," ").trim().trim_end().to_string();
       
       while a<query.len() {
         let cote = query.char_indices().nth(a).unwrap().1;
         let mut check = false;
         if cote=='\''{
             if start == 0{
                 start = a.clone();
             }
             else if start!=0{
                 end = a.clone();
             }
         }
         if start !=0 && end !=0{
            let mut d = query[start..end+1].to_string();
            d = d.replace("'"," ");
            d = d.trim().trim_end().to_string();
            d = d.replace(" ","!@#880");
            new_query.push_str(&d);
            start = 0;
            end = 0;
            check = true;
            } 
            if start ==0 && !check{
             new_query.push_str(&cote.to_string());
            }
         a = a+1;
     }
     query = new_query;
       
       if query == "exit"{
            return;
        }
        else{
         let l_data:Vec<&str> = query.trim().trim_end().split(';').collect();
         for i in l_data{
             querys = vec![];
             querys.push(i.trim().trim_end().to_string());
         }
         //now include all the instructions here
         for i in querys.iter(){
             let splited_query:Vec<&str> = i.split('>').collect();
             // we have to use some storage conditions in order to perform the executions
            let all_query:Vec<&str> = splited_query[0].split(' ').collect();
      
              if all_query[0] == "GET"{
              
                 //GET FROM users IN icommunity where {}
                  //this dml is hybrid one.
                  //conditions to add => single relational operator
                  if all_query[1] == "FROM" && all_query[3] == "IN"{
                      let class = all_query[2].to_string();
                      let storage = all_query[4].to_string();
                      unsafe{

                        let lin_search = check_class_storage(storage, class);
                        if lin_search.0{
                            let index = lin_search.1;
                            let part =   dml(i.to_string(),index);
                            VECTORS.push(part);
                            if splited_query.len()>1{
                                for i in 1..splited_query.len(){
                                    let last_split:Vec<&str> = splited_query[i].trim().trim_end().split(' ').collect();
                                    if last_split[0] == "STORE"{
                                      store(last_split[1]);
                                      
                                       // println!("location {:?} value {:?}",LOCATION,LOCATION_STORAGE);
                                      
                                    }
                                    else if last_split[0] == "STORE_UPDATE"{
                                      store_update(last_split[1]);

                                    } 
                                   // else{
                                     //   println!("WE DON'T KNOW");
                                    //}
                                }
                              }
                              let return_indicator:Vec<&str> = i.split(">>").collect();
                              if return_indicator.len() == 2{
                                 //split it and check it.
                                 if return_indicator[1].len()>3{
                                     if return_indicator[1][0..4].to_string() == "RFV("{
                                         //this is going to be tough
                                         println!("{:?}",return_indicator[1][0..4].to_string());
                                         let d_return_indicator = rfv_(i.to_string());
                                         //check for name in functions
                                         
                                            let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                            if counter == 0{
                                                FUNCR.push(d_return_indicator);
                                            }
                                            else{
                                                panic!("value with name {} already exists",d_return_indicator.0);
                                            }
                                         return;
                                     } 
                                     else if return_indicator[1][0..7].to_string() == "RETURN("{
                                        let d_return_indicator = return_(i.to_string());
                                        println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                        return;
                                     }
                                 }
                             }   
                        }
                        else{
                            panic!("Class Or Storage Not Found");
                        }

                        /*   match CLASS.binary_search(&(storage,class)){
                              Ok(index)=>{
                                 let part =   dml(i.to_string(),index);
                                   VECTORS.push(part);
                                   if splited_query.len()>1{
                                       for i in 1..splited_query.len(){
                                           let last_split:Vec<&str> = splited_query[i].trim().trim_end().split(' ').collect();
                                           if last_split[0] == "STORE"{
                                             store(last_split[1]);
                                             
                                              // println!("location {:?} value {:?}",LOCATION,LOCATION_STORAGE);
                                             
                                           }
                                           else if last_split[0] == "STORE_UPDATE"{
                                             store_update(last_split[1]);
 
                                           } 
                                          // else{
                                            //   println!("WE DON'T KNOW");
                                           //}
                                       }
                                     }
                                     let return_indicator:Vec<&str> = i.split(">>").collect();
                                     if return_indicator.len() == 2{
                                        //split it and check it.
                                        if return_indicator[1].len()>3{
                                            if return_indicator[1][0..4].to_string() == "RFV("{
                                                //this is going to be tough
                                                println!("{:?}",return_indicator[1][0..4].to_string());
                                                let d_return_indicator = rfv_(i.to_string());
                                                //check for name in functions
                                                
                                                   let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                                   if counter == 0{
                                                       FUNCR.push(d_return_indicator);
                                                   }
                                                   else{
                                                       panic!("value with name {} already exists",d_return_indicator.0);
                                                   }
                                                return;
                                            } 
                                            else if return_indicator[1][0..7].to_string() == "RETURN("{
                                               let d_return_indicator = return_(i.to_string());
                                               println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                               return;
                                            }
                                        }
                                    }                            
                              },
                              Err(_)=>println!("CLASS OR STORAGE NOT FOUND"),
                          }*/
                      }
 
                  }
              }
              else if all_query[0] == "INSERT"{
              //INSERT INTO ABC IN DEF -> [{key:value}|{key1:value1}|{key3:value3}|{key4:value4}]
              let splitter:Vec<&str> = i.split("->").collect();
              let spacer:Vec<&str> = splitter[0].split(" ").collect();
              
              if spacer[1] == "INTO" && spacer[3] == "IN" && splitter.len()==2{
                 //check for class & storage existence  
                 let class_name = spacer[2].trim().trim_end().to_string();
                 let storage_name = spacer[4].trim().trim_end().to_string();

                 unsafe{
                     //check for the name existence
                     let lin_search = check_class_storage(storage_name.clone(), class_name.clone());
                     if lin_search.0{
                         let index = lin_search.1;
                        let mut slice = splited_query[1].trim().trim().to_string(); 
                        slice = slice.replace("[","");
                        slice = slice.replace("]","");
                        slice = slice.trim().to_string();
                        let data:Vec<&str> = slice.split('|').collect();
                        let mut g_object = OBJECTS[index].clone();
                        if g_object.0 == class_name{
                            let returned_tuple = insert::insert_new(data,g_object.1.0.clone(),g_object.1.1.clone(),INPUT_STORAGE.clone());
                            //println!("{:?} ov {:?}",returned_tuple,g_object.clone());
                            //println!("{:?}",returned_tuple);
                            if returned_tuple.0{
                                g_object.1.1 = returned_tuple.1;
                                OBJECTS[index] = g_object;
                                VECTORS.push(vec!["true".to_string()]);
                            }
                            else{
                                VECTORS.push(vec!["false".to_string()]);
                                //A new Scenerio HAs Been Created
                                println!("CHECK THE COLUMN NAMES");
                            }
                            //println!("{:?}",OBJECTS);
                        }
                //        println!("DATABASE WITH NAME {} ALREADY EXISTS",store_name);
                     }
                     else{
                         panic!("CLASS OR STORAGE NOT FOUND");
                        }/* 
                     match CLASS.binary_search(&(storage_name.clone(),class_name.clone())){
                         Ok(index)=>{
                             let mut slice = splited_query[1].trim().trim().to_string(); 
                             slice = slice.replace("[","");
                             slice = slice.replace("]","");
                             slice = slice.trim().to_string();
                             let data:Vec<&str> = slice.split('|').collect();
                             let mut g_object = OBJECTS[index].clone();
                             if g_object.0 == class_name{
                                 let returned_tuple = insert::insert_new(data,g_object.1.0.clone(),g_object.1.1.clone(),INPUT_STORAGE.clone());
                                 //println!("{:?} ov {:?}",returned_tuple,g_object.clone());
                                 //println!("{:?}",returned_tuple);
                                 if returned_tuple.0{
                                     g_object.1.1 = returned_tuple.1;
                                     OBJECTS[index] = g_object;
                                     VECTORS.push(vec!["true".to_string()]);
                                 }
                                 else{
                                     VECTORS.push(vec!["false".to_string()]);
                                     //A new Scenerio HAs Been Created
                                     println!("CHECK THE COLUMN NAMES");
                                 }
                                 //println!("{:?}",OBJECTS);
                             }
 
                     //        println!("DATABASE WITH NAME {} ALREADY EXISTS",store_name);
                         }
                         Err(_)=>{
                             println!("STORAGE OR CLASS DOES NOT EXISTS");
                         }
                     }*/

                                     let mut return_indicator:Vec<&str> = i.split(">>").collect();
                                    // println!("return_i {:?}",return_indicator);
                                     if return_indicator.len() == 2{
                                        //split it and check it.
                                        if return_indicator[1].len()>3{
                                            return_indicator[1] = return_indicator[1].trim().trim_end();
                                            if return_indicator[1][0..4].to_string() == "RFV("{
                                                //this is going to be tough
                                                let d_return_indicator = rfv_(i.to_string());
                                                //check for name in functions
                                                
                                                   let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                                   if counter == 0{
                                                       FUNCR.push(d_return_indicator);
                                                   }
                                                   else{
                                                       panic!("value with name {} already exists",d_return_indicator.0);
                                                   }
                                                   return;
                                            } 
                                            else if return_indicator[1][0..7].to_string() == "RETURN("{
                                               let d_return_indicator = return_(i.to_string());
                                               println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                               return;
                                            }
                                        }
                                    }    
                   }  
                 
                 
            
              }
              else{
                  //you might have the syntax error
              }
              }
              else if all_query[0] == "INPUT"{
                  let splitter:Vec<&str> = i.split("->").collect();
                  if splitter.len()==2{
                      let mut string = splitter[1].to_string();
                      string = string.trim().trim_end().to_string();
                      //println!("{:?} AND {:?}",string.chars().nth(0),string.chars().nth(string.len()-1));
                       if string.chars().nth(0) == Some('[') && string.chars().nth(string.len()-1) == Some(']'){
                           string = string.replace("[","");
                           string = string.replace("]","");
                           let string_vec:Vec<&str> = string.split(',').collect();
                           if string_vec.len()>0 && string_vec[0] != ""{
                               let d_vec = string_vec.clone();
                               for i in string_vec{
                                   //check for duplicates.
                                   let cc = af::count_freq(d_vec.clone(),i);
                                   if cc == 1{
                                       //ask for input
                                      println!("{}:",i);
                                       let mut value = String::from("");
                                       //value = io::stdin().read_line(&mut value).expect("error while Data Input").to_string();
                                       match io::stdin().read_line(&mut value){
                                           Ok(_)=>{
 
                                           }
                                           Err(_)=>{
                                               panic!("INPUT SYSTEM ERROR");
                                           }
                                       }
                                       unsafe{
                                         INPUT_STORAGE.push((i.to_string(),value.replace("\n","")));
                                       }
                                   }
                               }
 
                           }
                           else{
                               panic!("Check The INPUTS");
                           }
 
                       }
                       else{
                           panic!("Check For Statement '[' AND ']' tokens");
                       }
                  } 
                  else{
                      panic!("PLEASE CHECK THE SYNTAX FOR 'INPUT' STATEMENTS");
                  }
              }
              else if all_query[0] == "CMP"{
                  /*
                  1.AXC(key)
                  2.ITR<c<key
                  3.INPUT(Index)
                  4.NORMAL
                  Step1:COLLECT KEYS,VALUES,RO,CO in different vectors inside a single part.
                  */
                 unsafe{
                     let data =  cmp_fn(i.to_string());
                   //  println!("{:?}",data);
                     let functions:Vec<&str> = i.split(">>").collect();
                             /*
                             1.STORE->(AXC,ITR)
                             2.EXIT->(SIMPLY EXIT THE PROGRAME)
                             3.RUN->(RUN ANY INSTRUCTION SET BASED ON NAME AND OWNERSHIP ONLY BY GIVING THE NAME)
                             */
                     if data.len() > 0{
                         //execute the true_command;
                         if functions.len()>1{
                             let true_command = functions[1].trim().trim_end();
                             //we will run a loop to calculate the total number of functions requires.
                             let splitted_commands:Vec<&str> = true_command.split("||").collect();
                             //start changing the stuffs

                             for i in splitted_commands{
                                 let i_dip = i.trim().trim_end();
                                 if i_dip.len()>4{
                                    if i_dip[0..5].to_string() == "STORE"{
 
                                        let command = i_dip[5..].trim().trim_end();
                                          store(command);
                                    }
                                    else if i_dip[0..4].to_string() == "RFV("{
                                        //create functions for it
                                        let d = cmp_rfv(i_dip.to_string());
                                        
                                            let counter = af::count_freq_tuple_thrice(FUNCR.clone(),d.0.clone());
                                            if counter == 0{
                                                FUNCR.push(d);
                                            }
                                            else{
                                                panic!("value with key {} already exists",d.0);
                                            }
                                            return;
                                    }
                                    else if i_dip[0..5].to_string() == "FUNC("{
                                        let mut func_name = i_dip.replace("FUNC(","");
                                        func_name = func_name.replace(")","");
                                        //now we have the key, using the key get the instructions and send it to the instruction_executer;
                                        
                                            let instructions = af::count_freq_tuple_return(FUNCTION.clone(),func_name.clone());
                                            if instructions != ""{
                                                instruction_executer(instructions);
                                            } 
                                            else{
                                                panic!("Either Function {} not exist Or Instructions Inside The FUnction Is Empty",func_name);
                                            }
                                        
                                    }
                                    else if i_dip.len()>6{
                                         if i_dip[0..7].to_string() == "RETURN("{
                                            let d_return_indicator = cmp_return(i_dip.to_string());
                                            println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                            return;
                                        }
                                        else if i_dip.len()>10{
                                             if i_dip[0..11].to_string() == "STORE_UPDATE"{
                                                let command = i[11..].trim().trim_end();
                                                store_update(command);
                                            }
                                        }
                                    }
                                  //  else if i[0..3].to_string() == "RUN"{
                                        //We will see
                                    //}
                                 }
                                  //Mark These Points For Future Use
                             //YOU CAN DIVIDE FUNCTIONS USING || AND CAN RUN MULTIPLE FUNCTIONS
                         }
                     }
                    } 
                     else if data.len() == 0 {
                         //execute the false command
                         if functions.len()>2{
                             let false_command = functions[2];
                             let splitted_commands:Vec<&str> = false_command.split("||").collect();
                           //  if splitted_commands.len()>0{
                                 for i in splitted_commands{
                                    if i.len()>4{
                                        let i_dip = i.trim().trim_end().to_string();
                                        if i_dip[0..5].to_string() == "STORE"{
     
                                            let command = i_dip[5..].trim().trim_end();
                                              store(command);
                                        }
                                        else if i_dip[0..4].to_string() == "RFV("{
                                            //create functions for it
                                            let d = cmp_rfv(i_dip.to_string());
                                            
                                                let counter = af::count_freq_tuple_thrice(FUNCR.clone(),d.0.clone());
                                                if counter == 0{
                                                    FUNCR.push(d);
                                                }
                                                else{
                                                    panic!("value with key {} already exists",d.0);
                                                }
                                                return;
                                            
    
                                        }
                                        else if i_dip[0..5].to_string() == "FUNC("{
                                            let mut func_name = i_dip.replace("FUNC(","");
                                            func_name = func_name.replace(")","");
                                            //now we have the key, using the key get the instructions and send it to the instruction_executer;
                                            
                                                let instructions = af::count_freq_tuple_return(FUNCTION.clone(),func_name.clone());
                                                if instructions != ""{
                                                    instruction_executer(instructions);
                                                } 
                                                else{
                                                    panic!("Either Function {} not exist Or Instructions Inside The FUnction Is Empty",func_name);
                                                }
                                            
                                        }
                                        else if i_dip.len()>6{
                                             if i_dip[0..7].to_string() == "RETURN("{
                                                let d_return_indicator = cmp_return(i_dip.to_string());
                                                println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                                return;
                                            }
                                            else if i_dip.len()>10{
                                                 if i_dip[0..11].to_string() == "STORE_UPDATE"{
                                                    let command = i_dip[11..].trim().trim_end();
                                                    store_update(command);
                                                }
                                            }
                                        }
                                      //  else if i[0..3].to_string() == "RUN"{
                                            //We will see
                                        //}
                                     }
                            }

                        
                         }
                     }
                     VECTORS.push(data);
                 }
              }
             
              else if all_query[0] == "UPDATE"{
                  //UPDATE students IN schooldiary
                  let split_update:Vec<&str> = i.split("SET").collect();
                  //println!("{} {}",all_query[1],all_query[3]);
                  if split_update.len()==2 &&  all_query[2]== "IN"{
                      let class = all_query[1].to_string();
                      let storage = all_query[3].to_string();
                      unsafe{
                          let lin_search = check_class_storage(storage, class);
                          if lin_search.0{
                              let index = lin_search.1;
                              let h =update_one(split_update[0].to_string(),index);
                              if h.len()>0{
                     
                                           for i in 0..h.len(){
                                            update_second(split_update[1].to_string(),index,i);
                                             }
                              }
                              else{
                                   print!("UPDATE CONDITIONS NOT SATISFIED");
                                  }
                          }
                          else{
                              panic!("Class Or Storage Not Found");
                          }
/* 
                          match CLASS.binary_search(&(storage,class)){
                              Ok(index)=>{
                                  //lets go ahead
                                 
                              }
                              Err(_)=>{
                                  println!("STORAGE OR CLASS NOT FOUND");
                              }
                          }
                          */
                          //call here
                          let mut return_indicator:Vec<&str> = i.split(">>").collect();
                          if return_indicator.len() == 2{
                            //split it and check it.
                            if return_indicator[1].len()>3{
                                return_indicator[1] = return_indicator[1].trim().trim_end();
                                if return_indicator[1][0..4].to_string() == "RFV("{
                                    //this is going to be tough
                                    let d_return_indicator = rfv_(i.to_string());
                                    //check for name in functions
                                    
                                       let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                       if counter == 0{
                                           FUNCR.push(d_return_indicator);
                                       }
                                       else{
                                           panic!("value with name {} already exists",d_return_indicator.0);
                                       }
                                       return;
                                } 
                                else if return_indicator[1][0..7].to_string() == "RETURN("{
                                   let d_return_indicator = return_(i.to_string());
                                   println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                   return;
                                }
                            }
                        }  
                          
                      }
 
                     
                  }
                  else{
                      println!("INSTRUCTION IS NOT AS PER THE ALIOTH STATEMENTS");
                  }
 
              }
              else if all_query[0] == "DELETE"{
                 //let split_update:Vec<&str> = i.split("SET").collect();
                  if  all_query[2]== "IN"{
                      let class = all_query[1].to_string();
                      let storage = all_query[3].to_string();
                      unsafe{
                          let lin_search = check_class_storage(storage, class);
                          if lin_search.0{
                              let index = lin_search.1;
                              let h =update_one(i.to_string(),index);
                              if h.len()>0{
                     
                                           for i in 0..h.len(){
                                            delete(index,i);
                                             }
                              }
                              else{
                                   print!("DELETE CONDITIONS NOT SATISFIED");
                                  }
                          }
                          else{
                              panic!("Storage Or Class Does Not Exists");
                          }
                          /* 
                          match CLASS.binary_search(&(storage,class)){
                              Ok(index)=>{
                                  //lets go ahead
                                
                              }
                              Err(_)=>{
                                  println!("STORAGE OR CLASS NOT FOUND");
                              }
                          }
                          */
                          let mut return_indicator:Vec<&str> = i.split(">>").collect();
                          if return_indicator.len() == 2{
                            //split it and check it.
                            if return_indicator[1].len()>3{
                                return_indicator[1] = return_indicator[1].trim().trim_end();
                                if return_indicator[1][0..4].to_string() == "RFV("{
                                    //this is going to be tough
                                    let d_return_indicator = rfv_(i.to_string());
                                    //check for name in functions
                                    
                                       let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                       if counter == 0{
                                           FUNCR.push(d_return_indicator);
                                       }
                                       else{
                                           panic!("value with name {} already exists",d_return_indicator.0);
                                       }
                                       return;
                                } 
                                else if return_indicator[1][0..7].to_string() == "RETURN("{
                                   let d_return_indicator = return_(i.to_string());
                                   println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                   return;
                                }
                            }
                        }  
                      }
                  }
                  else{
                      println!("INSTRUCTION IS NOT AS PER THE ALIOTH STATEMENTS");
                  }
              }
              else if  all_query[0] == "CREATE" {
                 let data_for:Vec<&str> = i.split("->").collect();
                 let data_one:Vec<&str> = data_for[0].split(" ").collect();
                 if data_one[1] == "STORAGE"{
                     if data_one.len()>1{
                         let mut store_name = data_one[2];
                         let mut trial_name = "".to_string();
                         if store_name.len()>3{
                             if store_name[0..4].to_string() == "AXC("{
                                 trial_name = store_name.replace("AXC(","");
                                 trial_name = trial_name.replace(")","");
                                 trial_name = access_value(trial_name);
                                 store_name = &trial_name;
 
                             }
                             else if store_name[0..4].to_string() == "ITR<"{
                                 let splitted:Vec<&str> = store_name.split("<").collect();
                                 if splitted.len()==3{
                                     //LETS HAVE IT
                                     unsafe{
                                         let index_count:usize = splited_query[1].parse().unwrap();
                                         if index_count<VECTORS.len(){
                                             let get = to_get(VECTORS[index_count].clone(),splitted[2]);
                                             trial_name = get[get.len()-1].clone();
                                             store_name = &trial_name;
                                         }
                                         else{
                                             println!("DATA NOT FOUND OR STATEMENT IS NOT CORRECT");
                                         }
                                     }
                                     
                                 }
                             }
                         }
                         else if store_name.len()>7{
                             //this will go for the concate
                             if store_name[0..7].to_string() == "CONCATE"{
                                 trial_name = concate(store_name.to_string());
                                 store_name = &trial_name;
                             }
                         }
                         unsafe{
                             //check for the name existence
                             match STORAGE.binary_search(&store_name.to_string()){
                                 Ok(_)=>{
                                     println!("DATABASE WITH NAME {} ALREADY EXISTS",store_name);
                                 }
                                 Err(_)=>{
                                     STORAGE.push(store_name.to_string());
                                     println!("STORAGE CREATED ON CHAIN JADN");
                                 }
                             }
                           }  
                     }
                 }
                 else if data_one[1] == "CLASS" {
                     //CREATE CLASS players IN digi
                     if data_one.len() > 5{
                         let mut class_name = data_one[2];
                         if data_one[3] == "IN" && data_for.len() == 2{
                             let mut storage_name = data_one[4];
 
                             //Parse The Query And RESET THE VALUES INSIDE THE DATA.
 
                             let mut trial_name = "".to_string();
                             if storage_name.len()>3{
                                 if storage_name[0..4].to_string() == "AXC("{
                                     trial_name = storage_name.replace("AXC(","");
                                     trial_name = trial_name.replace(")","");
                                     trial_name = access_value(trial_name);
                                     storage_name = &trial_name;
     
                                 }
                                 else if storage_name[0..4].to_string() == "ITR<"{
                                     let splitted:Vec<&str> = storage_name.split("<").collect();
                                     if splitted.len()==3{
                                         //LETS HAVE IT
                                         unsafe{
                                             let index_count:usize = splited_query[1].parse().unwrap();
                                             if index_count<VECTORS.len(){
                                                 let get = to_get(VECTORS[index_count].clone(),splitted[2]);
                                                 trial_name = get[get.len()-1].clone();
                                                 storage_name = &trial_name;
                                             }
                                             else{
                                                 println!("DATA NOT FOUND OR STATEMENT IS NOT CORRECT");
                                             }
                                         }
                                         
                                     }
                                 }
                             }
                             else if storage_name.len()>7{
                                 //this will go for the concate
                                 if storage_name[0..7].to_string() == "CONCATE"{
                                     trial_name = concate(storage_name.to_string());
                                     storage_name = &trial_name;
                                 }
                             }
                             //PARSING FOR CLASS NAME
                             let mut trial_name = "".to_string();
                             if class_name.len()>3{
                                 if class_name[0..4].to_string() == "AXC("{
                                     trial_name = class_name.replace("AXC(","");
                                     trial_name = trial_name.replace(")","");
                                     trial_name = access_value(trial_name);
                                     class_name = &trial_name;
                                 }
                                 else if class_name[0..4].to_string() == "ITR<"{
                                     let splitted:Vec<&str> = class_name.split("<").collect();
                                     if splitted.len()==3{
                                         //LETS HAVE IT
                                         unsafe{
                                             let index_count:usize = splited_query[1].parse().unwrap();
                                             if index_count<VECTORS.len(){
                                                 let get = to_get(VECTORS[index_count].clone(),splitted[2]);
                                                 trial_name = get[get.len()-1].clone();
                                                 class_name = &trial_name;
                                             }
                                             else{
                                                 println!("DATA NOT FOUND OR STATEMENT IS NOT CORRECT");
                                             }
                                         }
                                         
                                     }
                                 }
                             }
                             else if class_name.len()>7{
                                 //this will go for the concate
                                 if class_name[0..7].to_string() == "CONCATE"{
                                     trial_name = concate(class_name.to_string());
                                     class_name = &trial_name;
                                 }
                             }
                             //search for STORAGE EXISTENCE
                             unsafe{
                                 match STORAGE.binary_search(&storage_name.to_string()){
                                     Ok(_) =>{
                                         match CLASS.binary_search(&(storage_name.to_string(),class_name.to_string())){
                                             Ok(_)=>{
                                                 //THROW ERROR.
                                                 println!("CLASS WITH NAME {} AlREADY EXIST IN STORAGE {}",class_name,storage_name);
                                             }
                                             Err(_)=>{
                                                 //PARSING FOR COLUMN VALUES
                                                 let mut data = data_for[1].replace("{"," ");
                                                  data = data.replace("}"," ");
                                                  data = data.trim().trim_end().to_string();
                                                  let vec_data = insert::insert_column(data.split(",").collect());
                                                     CLASS.push((storage_name.to_string(),class_name.to_string()));
                                                     OBJECTS.push((class_name.to_string(),(vec_data,vec![])));
                                                 println!("CLASS CREATED IN STORAGE {}",storage_name);
                                             }
                                         }
                                     },
                                     Err(_)=>{
                                         println!("STOARGE {} NOT EXISTS",storage_name);
                                     }
                                 }
                             }
                         }
                         else{
                             println!("IN Statement Missing OR THE '->' Statement Missing");
                         }
                     }
 
                 }
                 /*
                  1.Create STORE
                  2.Create OB
                  */
                 let return_indicator:Vec<&str> = i.split(">>").collect();
                                     if return_indicator.len() == 2{
                                         //split it and check it.
                                         if return_indicator[1].len()>3{
                                             if return_indicator[1][0..4].to_string() == "RFV("{
                                                 //this is going to be tough
                                                 let d_return_indicator = rfv_(i.to_string());
                                                 //check for name in functions
                                                 unsafe{
                                                    let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                                                    if counter == 0{
                                                        FUNCR.push(d_return_indicator);
                                                    }
                                                    else{
                                                        panic!("value with name {} already exists",d_return_indicator.0);
                                                    }
                                                 }
                                                 return;
                                             } 
                                             else if return_indicator[1][0..7].to_string() == "RETURN("{
                                                let d_return_indicator = return_(i.to_string());
                                                println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                                                return;
                                             }
                                         }
                                     } 
                  
              }
              else if all_query[0] == "STORE"{
                 // println!("{:?}",all_query[1]);
                  store(all_query[1]);
                  let return_indicator:Vec<&str> = i.split(">>").collect();
                  if return_indicator.len() == 2{
                    //split it and check it.
                    if return_indicator[1].len()>3{
                        if return_indicator[1][0..4].to_string() == "RFV("{
                            //this is going to be tough
                            let d_return_indicator = rfv_(i.to_string());
                            //check for name in functions
                            unsafe{
                               let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                               if counter == 0{
                                   FUNCR.push(d_return_indicator);
                               }
                               else{
                                   panic!("value with name {} already exists",d_return_indicator.0);
                               }
                            }
                            return;
                        } 
                        else if return_indicator[1][0..7].to_string() == "RETURN("{
                           let d_return_indicator = return_(i.to_string());
                           println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                           return;
                        }
                    }
                } 
              }
              else if all_query[0] == "STORE_UPDATE"{
                  store_update(all_query[1]);
                  let mut return_indicator:Vec<&str> = i.split(">>").collect();
                  if return_indicator.len() == 2{
                    //split it and check it.
                    return_indicator[1] = return_indicator[1].trim().trim_end();
                    if return_indicator[1].len()>3{
                        if return_indicator[1][0..4].to_string() == "RFV("{
                            //this is going to be tough
                            let d_return_indicator = rfv_(i.to_string());
                            //check for name in functions
                            unsafe{
                               let counter =  af::count_freq_tuple_thrice(FUNCR.clone(),d_return_indicator.0.clone());
                               if counter == 0{
                                   FUNCR.push(d_return_indicator);
                               }
                               else{
                                   panic!("value with name {} already exists",d_return_indicator.0);
                               }
                            }
                            return;
                        } 
                        else if return_indicator[1][0..7].to_string() == "RETURN("{
                           let d_return_indicator = return_(i.to_string());
                           println!("Message {:?} Vector {:?}",d_return_indicator.0,d_return_indicator.1);
                           return;
                        }
                    }
                } 
              }
            else if all_query[0].len()>4 && all_query[0][0..5].to_string() == "FUNC("{
                //now we have to check the value of the functions by calling first removeing FUNC( and ')' and send the instructions to the instructions executers
                let mut func_name = i.replace("FUNC(","");
                func_name = func_name.replace(")","");
                //now we have the key, using the key get the instructions and send it to the instruction_executer;
                unsafe{
                    let instructions = af::count_freq_tuple_return(FUNCTION.clone(),func_name.clone());
                    if instructions != ""{
                        instruction_executer(instructions);
                    } 
                    else{
                        panic!("Either Function {} not exist Or Instructions Inside The FUnction Is Empty",func_name);
                    }
                }
            }
         }
        //summary of datas
        /* 
        unsafe{
          println!("{:?}",VECTORS);
      
        }
        */
        }
    }
    //let mut vectors_of_data:Vec<&str> =vec![] ;
}

fn afv(key:String)->(String,String,Vec<Vec<String>>){
    unsafe{
        
        let mut value_to_send:Vec<(String,String,Vec<Vec<String>>)> =vec![];
        let mut check = false;
        for i in FUNCR.clone(){
        //    println!("{:?} == {:?}",i.0,key);
            if i.0 == key{
                check=true;
                value_to_send.push(i);
                break;
            }
        }
        if check{
            value_to_send[0].clone()
        }
        else{
            panic!("Key {} Not Found",key);
        }
    }
}

fn check_class_storage(storage:String,class:String) ->(bool,usize){
    unsafe{
        let mut check = false;
        let mut index = 0;
        for i in 0..CLASS.clone().len(){
            if CLASS[i].0 == storage && CLASS[i].1 == class{
                check = true;
                index = i;
                break
            }
        }
        (check,index)
    }
}