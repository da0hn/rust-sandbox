fn has_unique_characters(input: &str) -> bool {
    let mut char_list = Vec::<char>::new();

    for char in input.chars() {
        if char_list.contains(&char) {
            return false;
        }
        char_list.push(char);
    }
    return true;
}

fn main() {
    println!("has unique characters: {}", has_unique_characters("Gabriel"));
    println!("has unique characters: {}", has_unique_characters("Abacate"));
    println!("has unique characters: {}", has_unique_characters("Mamão"));
}
