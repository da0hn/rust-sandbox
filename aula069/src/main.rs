fn is_anagram(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut left_chars: Vec<char> = left.chars().collect();
    let mut right_chars: Vec<char> = right.chars().collect();

    left_chars.sort();
    right_chars.sort();

    left_chars == right_chars
}

fn main() {
    println!("Is anagram: {}", is_anagram("listen", "silent"));
    println!("Is anagram: {}", is_anagram("hello", "world"));
    println!("Is anagram: {}", is_anagram("rust", "rust"));
    println!("Is anagram: {}", is_anagram("hack", "khac"));
}
