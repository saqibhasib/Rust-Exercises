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
        list.push(&input[last..len-2]);
    }

    return list;
}

fn calculate(_list: Vec<&str>) -> i64{
    return 44;
}

fn main(){
    let stdin = io::stdin();
    let mut input = String::new();
    
    match stdin.read_line(&mut input) {
        Ok(_n) => {
            println!("{}", input);
            let list: Vec<&str> = process_input(&input);
            println!("{:?}", list);
            let result: i64 = calculate(list);
            println!("{result}");
        }
        Err(_error) => {
            println!("error found: {_error}");
        }
    }
}