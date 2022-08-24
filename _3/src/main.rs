use std::{fs, collections::HashMap};

fn main() {
        // get value from inputs
    let inputs = fs::read_to_string("./inputs").unwrap();
    // parse inputs by line/ \n
    let inputs = inputs.split("\n").collect::<Vec<&str>>();
    // parse inputs to tuple str & int for action and value
    let inputs: Vec<&[u8]> = inputs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < inputs.len() - 1)
        .map(|(_, v)| {
            v.as_bytes()
        })
        .collect();

    let len_of_bit = inputs[0].len(); // get sample bit len from first input
    let mut zero_bit_total: HashMap<usize, i32> = HashMap::new();
    for v in inputs.iter(){
        for z in 0..len_of_bit{
            if v[z] == 48{ // 48 === 0
                if zero_bit_total.contains_key(&z){
                    zero_bit_total.insert(z, zero_bit_total.get(&z).unwrap() + 1);
                } else {
                    zero_bit_total.insert(z, 1);
                }
            }
        }
    }

    let mut gamma_rate  = "".to_string();
    let mut epsilon_rate = "".to_string(); 

    let total_of_inputs = inputs.len();
    for v in 0..len_of_bit{
        let zero_bit_total_value = zero_bit_total.get(&v).unwrap();
        if zero_bit_total_value > &(total_of_inputs as i32 / 2){
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1")
        } else {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0")
        }
    }

    let gamma_rate_num = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_num = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("The result part 1 is: {}", gamma_rate_num * epsilon_rate_num);
}
