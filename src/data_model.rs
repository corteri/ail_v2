/*
pub fn send_model2()-> (Vec<String>,Vec<Vec<String>>){
    let data1 = "{name:n1,age:2}".to_string();
    let data2 = "{name:n2,age:2}".to_string();
    let data3 = "{name:n3,age:2}".to_string();
    let data4 = "{name:n4,age:2}".to_string();
    let data5 = "{name:n5,age:2}".to_string();
    let data6 = "{name:n6,age:2}".to_string();
    let data7 = "{name:n7,age:2}".to_string();
    let data8 = "{name:n8,age:2}".to_string();
    let data9 = "{name:n9,age:2}".to_string();
    let data10 = "{name:n10,age:2}".to_string();
    let data11 = "{name:n11,age:2}".to_string();
    let data12 = "{name:n12,age:2}".to_string();

    let data_model:Vec<String> = vec![data1,data2,data3,data4,data5,data6,data7,data8,data9,data10,data11,data12];

    let mut keys:Vec<String> = vec![];
    let mut keys_values:Vec<String> = vec![];
    
    let mut column:Vec<String> = vec![];
    let mut values:Vec<Vec<String>> = vec![];

    for i in 0..data_model.len()-1{
        let  o_str = &data_model[i];
        //let mut  data_str:Vec<&str> = data_model[i].split(':').collect();
       let o_str = &o_str.replace('{'," ");
        let o_str = &o_str.replace('}'," ");
        let data_str:Vec<&str> = o_str.split(',').collect();
        for j in data_str{
            let vec_data:Vec<&str> = j.split(':').collect();
            keys.push(vec_data[0].to_string());
            keys_values.push(vec_data[1].to_string());
        }
    }
        //we have to start the real game from here
        for i in 0..keys.len(){
            if column.len()>0{
                let inl = check_column(&keys[i].clone(), column.clone());
                if inl>=0{
                    let new_va = inl as usize;
                    let new_value = &mut values[new_va].clone();
                    new_value.push(keys_values[i].clone());
                    values[new_va] = new_value.to_vec();
                }
                else{
                    column.push(keys[i].clone());
                    values.push(vec![keys_values[i].clone()]);
                }
            }
            else{
                column.push(keys[i].clone());
                values.push(vec![keys_values[i].clone()]);
            }
        }
    

    (column,values)
}

fn check_column(column_value:&str,column:Vec<String>)->i32{

    let mut check = false;
    let mut index = 0;
    //println!("{}",column.len()-1);
    for i in 0..column.len(){
        if column[i] == column_value.to_string(){
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
*/