use std::collections::HashSet;

fn intersection(v1: &[i32], v2: &[i32]) -> HashSet<i32> {
    let mut unique_elements = HashSet::new();

    for number in v1 {
        if v2.contains(number) {
            unique_elements.insert(*number);
        }
    }

    unique_elements
}

fn main() {
    println!("[1, 2, 3], [2, 3, 4] -> {:?}", intersection(&[1, 2, 3], &[2, 3, 4])); // Saída: { 2, 3 }
    println!("[1, 2, 2, 1], [2, 2] -> {:?}", intersection(&[1, 2, 2, 1], &[2, 2])); // Saída: { 2 }
}
