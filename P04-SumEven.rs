use std::io;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);

    let input_parts = input.split_whitespace().map(| x | x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();

    let mut sum = 0;
    let _in = input_parts.into_iter().map(| x: i32 | if x % 2 == 0 { sum += x; } ).collect::<Vec<_>>();

    println!("{:?}", sum );
}