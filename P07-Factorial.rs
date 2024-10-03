use std::io;

fn factorial_loop(input: i64) -> i64 {
    let mut result = 1i64;

    for n in 1..=input {
        result = n * result;
    }

    return result;
}

fn factorial_recursive (input: i64) -> i64 {
    if (input == 0) {
        1
    }
    else { input * factorial_recursive(input - 1) }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);

    input = input.split_whitespace().collect();
    let input_num = input.parse::<i64>().unwrap();

    let result_loop = factorial_loop(input_num);

    let result_recursive = factorial_recursive(input_num);

    println!("result of loop: {:}", result_loop);

    println!("result of recursion: {result_recursive}");
}