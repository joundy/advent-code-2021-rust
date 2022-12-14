use std::fs;

fn main() {
    // get value from inputs
    let inputs = fs::read_to_string("./inputs").unwrap();
    // parse inputs by line/ \n
    let inputs = inputs.split("\n").collect::<Vec<&str>>();
    // parse inputs to integerr
    let inputs: Vec<i32> = inputs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < inputs.len() - 1)
        .map(|(_, v)| v.parse::<i32>().unwrap())
        .collect();

    let mut total = 0;
    for (i, v) in inputs.iter().enumerate(){
        if i == 0{
            continue;
        }

        let previous_value = inputs[i-1];
        let current_value = v;

        if current_value > &previous_value{
            total += 1;
        }
    }
    println!("Total incresed number: {}", total);

    let mut total = 0;
    for (i, _) in inputs.iter().enumerate(){
        if i == 0 {
            continue;
        }

        if i+3+1 > inputs.len() -1 {
            break;
        }

        let previous_value = &inputs[i..i+3].iter().sum::<i32>();
        let current_value = &inputs[i+1..i+3+1].iter().sum::<i32>();

        if current_value > previous_value{
            total += 1;
        }
    }
    println!("Total incresed measured number: {}", total);
}
