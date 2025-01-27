fn permutation_level1 (input: &str, current_index: usize, depth: usize, mut flag_array: Vec<bool>) {
    if(depth == input.len()){
        println!(" ");
        return;
    }
    flag_array[current_index] = true;
    print!("{}", input.as_bytes()[current_index] as char);
    for n in 0..input.len() {
        if (flag_array[n] == false || n != current_index){
            permutation_level1(input, n, depth+1, flag_array.clone());
        }
        // println!("");
    }
    flag_array[current_index] = false;
}

fn main () {
    let input: &str = "abcd";
    let mut flag_array = vec![false; input.len()];
    permutation_level1(input, 0, 0, flag_array);
}