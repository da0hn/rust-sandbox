use std::collections::HashSet;

fn contains_duplicates(vec: &[i32]) -> bool {
    let mut unique_vec = HashSet::<i32>::new();

    for &i in vec {
        if !unique_vec.insert(i) {
            return true;
        }
    }
    false
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [1, 2, 3, 4, 1];

    println!("{:?} contains duplicates? {}", v1, contains_duplicates(&v1));
    println!("{:?} contains duplicates? {}", v2, contains_duplicates(&v2));
}
