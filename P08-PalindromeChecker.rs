fn palindrome_checker_rec (l: usize, r: usize, input: &str) -> bool {
    if l >= r {
        return true;
    }
    if input.get(l..=l) == input.get(r..=r) {
        return palindrome_checker_rec(l+1, r-1, input);
    }
    else {
        return false;
    }
}

fn main() {
    let input: &str = "ananda";
    let is_palindrome: bool = palindrome_checker_rec(0, (|| if !(input.is_empty()) { return input.len()-1; } else { return 0; })(), input);
    println!("{}", is_palindrome);
}