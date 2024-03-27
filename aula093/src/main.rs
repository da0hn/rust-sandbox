use std::collections::HashMap;

fn can_construct(ransom_note: &str, magazine: &str) -> bool {
    let ransom_note_size = ransom_note.chars().count();
    let magazine_size = magazine.chars().count();

    let length_restriction = 1..=105;

    if !length_restriction.contains(&ransom_note_size) || !length_restriction.contains(&magazine_size) { return false; }

    let mut magazine_map = HashMap::<char, i32>::new();
    for ch in magazine.chars() {
        *magazine_map.entry(ch).or_insert(1) += 1;
    }

    for ch in ransom_note.chars() {
        if let Some(count) = magazine_map.get_mut(&ch) {
            if *count == 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }
    true
}

fn main() {
    println!("ransom_note: a, magazine: b = {}", can_construct("a", "b")); // false
    println!("ransom_note: aa, magazine: abc = {}", can_construct("aa", "abc")); // false
    println!("ransom_note: aabd, magazine: aabbcd = {}", can_construct("aabd", "aabbcd")); // true
}
