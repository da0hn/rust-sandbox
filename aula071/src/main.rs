fn is_palindrome(input: &str) -> bool {
    let cleaned_str = input.to_string().to_lowercase().replace(" ", "");
    let reversed: String = cleaned_str.chars().rev().collect();
    reversed == cleaned_str
}

fn main() {
    println!("Is anagram: {}", is_palindrome("Mega Bobagem"));
    println!("Is anagram: {}", is_palindrome("Roma me Tem amor"));
    println!("Is anagram: {}", is_palindrome("A cara Rajada da Jararaca"));
    println!("Is anagram: {}", is_palindrome("hello world"));
}
