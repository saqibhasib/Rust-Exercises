use std::io;

// Global variables
let mut list_global: Vec<&str> = None;


fn process_input(input: &str) -> Vec<&str>{
    let mut list = Vec::new();
    let mut last = 0;

    for (index, matched) in input.match_indices(|c: char| (c == '+' || c == '-' || c == '*' || c == '/')){
        if last != index {
            list.push(&input[last..index]);
        }
        list.push(matched);
        last = index + matched.len();
    }
    let len = input.len();
    if last < len {
        // TODO: WHY -2??? Ans: "\n\r"
        list.push(&input[last..len-2]);
    }

    return list;
}



//Level 1 calculation function
fn calculate_1(_list: Vec<&str>) -> f32{
    let num1: f32 = _list[0].parse().unwrap();
    let num2: f32 = _list[2].parse().unwrap();
    let result: f32;
    // println!("num1: {num1}; num2: {num2}");
    match _list[1] {
        "+" => result = num1 + num2,
        "-" => result = num1 - num2,
        "*" => result = num1 * num2,
        "/" => result = num1 / num2,
        _ => result = 0f32,
    }
    return result;
}

// L2 calc func
fn calculate_2(){
    println!("inside calculate_2");
}

fn main(){
    let mut input = String::new();
    let stdin = io::stdin();
    
    match stdin.read_line(&mut input) {
        Ok(_n) => {
            // println!("{}", input);

            // L1---------------
            // let list: Vec<&str> = process_input(&input); // L1
            // println!("{:?}", list);
            // let result: f32 = calculate_1(list); // L1
            // println!("{result}");

            // L2---------------
            unsafe{
                list_global = process_input(&input); // L2
            }
            calculate_2(list_global); // L2
            println!("{}", list_global[0]);
        }
        Err(_error) => {
            println!("error found: {_error}");
        }
    }
}