use std::io;

fn fibonacci_loop(n: i32) -> i64 {
    let mut result: i64 = 1;
    let mut prev: i64 = 0;
    for _i in 2..=n {
        let temp: i64 = result;
        result = result + prev;
        prev = temp;
    }
    return result;
}

fn fibonacci_recursion(n: i32) -> i64 {
    if(n <= 0){
        return 0;
    }
    if (n == 1){
        return 1;
    }
    return fibonacci_recursion(n-1) + fibonacci_recursion(n-2);
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);
    let input_string = input.split_whitespace().collect::<String>();

    let n: i32 = input_string.parse::<i32>().unwrap();

    let result: i64 = fibonacci_recursion(n);

    println!("converted: {result}");
}