use std::io;

fn process_input(input: String) -> (i32, i32) {
    let parts = input.split(" ").collect::<Vec<&str>>();
    
    let mut s_string = parts[0].to_string();
    s_string = s_string.split_whitespace().collect();
    let temp: i32 = s_string.parse::<i32>().unwrap();

    s_string = parts[1].to_string();
    s_string = s_string.split_whitespace().collect();
    let opt: i32 = s_string.parse::<i32>().unwrap();
    
    return (temp, opt);
}

fn convert((temp, opt): (i32, i32)) -> (f32, String) {
    if (opt == 1) {
        return (((temp as f32 - 32.0) * 5.0 / 9.0), "Celcius".to_string());
    }
    else if (opt == 2) {
        return (((temp as f32 * 9.0 / 5.0) + 32.0), "Fahrenheit".to_string());
    }
    return (0f32, "".to_string());
}

fn main(){
    println!("what temperature and which conversion direction do you need? 
    type temperature <space> option number and enter: 1. F to C, 2. C to F
    for example: '25 2' for 25 deg Celcius to Fahrenheit");

    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);

    let input_processed: (i32, i32) = process_input(input);
    let (converted, temp_type) = convert(input_processed);

    println!("Converted value: {:.2} degrees {:}", converted, temp_type);
}