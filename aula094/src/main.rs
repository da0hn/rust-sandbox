use std::collections::HashMap;

fn word_pattern(pattern: &str, str_val: &str) -> bool {
    
    let str_list = str_val.split_whitespace().collect::<Vec<&str>>();
    let pattern_list = pattern.chars().collect::<Vec<char>>();

    if pattern_list.len() != str_list.len() {
        return false;
    }

    let mut pattern_map = HashMap::<char, &str>::new();

    for idx in 0..pattern.chars().count() {
        let current_char = pattern_list[idx];
        let current_word = str_list[idx];

        if let Some(&word) = pattern_map.get(&current_char) {
            if word != current_word {
                return false;
            }
        } else {
            if pattern_map.values().any(|&registered_word| registered_word == current_word) {
                return false;
            }
            pattern_map.insert(current_char, current_word);
        }
    }
    true
}

fn main() {
    let pattern1 = "abba";
    let str1 = "dog cat cat dog";
    println!("Segue o padrão: {}", word_pattern(pattern1, str1)); // Saída: true

    let pattern2 = "abba";
    let str2 = "dog cat cat fish";
    println!("Segue o padrão: {}", word_pattern(pattern2, str2)); // Saída: false

    let pattern3 = "aaaa";
    let str3 = "dog cat cat dog";
    println!("Segue o padrão: {}", word_pattern(pattern3, str3)); // Saída: false

    let pattern4 = "abba";
    let str4 = "dog dog dog dog";
    println!("Segue o padrão: {}", word_pattern(pattern4, str4)); // Saída: false

    let pattern5 = "abbaa";
    let str5 = "dog cat cat dog";
    println!("Segue o padrão: {}", word_pattern(pattern5, str5)); // Saída: false

    let pattern6 = "abbc";
    let str6 = "dog cat cat fish";
    println!("Segue o padrão: {}", word_pattern(pattern6, str6)); // Saída: true
}
