
fn is_permutation(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for char in left.chars() {
        if !right.contains(char) {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("abc is permutation of cba: {}", is_permutation("abc", "cba"));
    println!("abc is permutation of cda: {}", is_permutation("abc", "cda"));
    println!("abc is permutation of cdab: {}", is_permutation("abc", "cdab"));
}
