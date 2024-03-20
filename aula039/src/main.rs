fn compress_string(input: &str) -> String {
    let mut result_str = String::new();
    let mut idx = 0;
    while idx < input.chars().count() {
        let ch = input.chars().nth(idx).unwrap();
        result_str.push(ch);
        let mut count = 0;
        while Some(ch) == input.chars().nth(idx) {
            count += 1;
            idx += 1;
        }
        result_str.push_str(&count.to_string());
    }

    match result_str.len() > input.len() {
        true => input.to_string(),
        false => result_str
    }
}

fn main() {
    println!("Original: aabcccccaaa, Compressed: {}", compress_string("aabcccccaaa"));
    println!("Original: aabcccCCaaa, Compressed: {}", compress_string("aabcccCCaaa"));
    println!("Original: abcdefgh, Compressed: {}", compress_string("abcdefgh"));
}
