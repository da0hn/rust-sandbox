fn is_one_edit_distance(left: &str, right: &str) -> bool {
    let mut distance = 0;
    let mut counter = 0;
    while counter < left.chars().count() || counter < right.chars().count() {
        if distance > 1 {
            return false;
        }
        let current_left_char = left.chars().nth(counter);
        let current_right_char = right.chars().nth(counter);
        if current_left_char != current_right_char {
            distance += 1;
        }
        counter += 1;
    }
    return distance == 1;
}

fn main() {
    println!("abc vs ab -> distance: {}", is_one_edit_distance("abc", "ab"));
    println!("pale vs bale -> distance: {}", is_one_edit_distance("pale", "bale"));
    println!("pales vs pale -> distance: {}", is_one_edit_distance("pales", "pale"));
    println!("bibo vs pale -> distance: {}", is_one_edit_distance("bibo", "pale"));
}
