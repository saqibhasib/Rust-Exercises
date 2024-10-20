use std::collections::HashMap;
use std::io;

fn word_frequency(input: &str) -> (String, i32) {
    let input_vec: Vec<&str> = input.split(| c: char | !(c.is_alphanumeric() || c == '\'')).filter(| s | !(s.is_empty())).collect();

    let mut histogram = HashMap::<&str, i32>::new();

    for i in input_vec {
        histogram.entry(i).and_modify(| n | *n += 1).or_insert( 1 );
    }

    let mut max_freq: i32 = 0;
    let mut max_key: &str = "";
    for (k, v) in histogram.iter() {
        if *v > max_freq {
            max_freq = *v;
            max_key = k;
        }
    }
    // println!("{:?}", input_vec);
    (max_key.to_string(), max_freq)
}

fn main() {
    let input: &str = "hi i am saqib. i am writing. i am";
    let (freq_word, frequency) = word_frequency(input);

    println!("'{freq_word}' appears {frequency} times.");
}