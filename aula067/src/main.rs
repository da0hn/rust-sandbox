fn reverse_string(str: &str) -> String {
    return str.to_string().chars().rev().collect();
}

fn main() {
    println!("Original: Hello World, Reversed: {}", reverse_string("Hello World"));
    println!("Original: Rust, Reversed: {}", reverse_string("Rust"));
}
