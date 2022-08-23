use std::fs;

fn main() {
    // get value from inputs
    let inputs = fs::read_to_string("./inputs").unwrap();
    // parse inputs by line/ \n
    let inputs = inputs.split("\n").collect::<Vec<&str>>();
    // parse inputs to tuple str & int for action and value
    let inputs: Vec<(&str, i32)> = inputs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < inputs.len() - 1)
        .map(|(_, v)| {
            let a: Vec<&str> = v.split(" ").collect();
            (a[0], a[1].parse::<i32>().unwrap())
        })
        .collect();

    // part 1
    let mut horizontal = 0;
    let mut depth = 0;
    for v in inputs.iter(){
        match v.0{
            "forward" => horizontal += v.1,
            "down" => depth += v.1,
            "up" => depth -= v.1,
            _ => panic!("errors.action")
        }
    }

    let result = horizontal * depth;
    println!("The result part 1 is: {}", result);

    // part 2
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for v in inputs.iter(){
        match v.0{
            "forward" => {
                horizontal += v.1;
                if aim != 0{
                    depth += aim * v.1;
                }
            },
            "down" => aim += v.1,
            "up" => aim -= v.1,
            _ => panic!("errors.action")
        }
    }

    let result = horizontal * depth;
    println!("The result part 2 is: {}", result);
}
