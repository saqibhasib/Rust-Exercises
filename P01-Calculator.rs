use std::io;

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
        // TODO: WHY -2???
        list.push(&input[last..len-2]);
    }

    return list;
}

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

fn main(){
    let mut input = String::new();
    let stdin = io::stdin();
    
    match stdin.read_line(&mut input) {
        Ok(_n) => {
            // println!("{}", input);
            let list: Vec<&str> = process_input(&input);
            // println!("{:?}", list);
            let result: f32 = calculate_1(list);
            println!("{result}");
        }
        Err(_error) => {
            println!("error found: {_error}");
        }
    }
}