fn three_sum_closest(array: &[i32], target: i32) -> i32 {
    let mut array = array.to_vec();
    array.sort();
    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;
    
    for i in 0..array.len()-2 {
        let mut left = i + 1;
        let mut right = array.len() - 1;
        while left < right {
            let current_sum = array[i] + array[left] + array[right];
            let current_diff = (current_sum - target).abs();
            if current_diff < min_diff {
                min_diff = current_diff;
                closest_sum = current_sum;
            }
            if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    closest_sum
}

fn main() {
    println!("Closest sum for array {:?} and target {} is {}", [-1, 2, 1, -4], 1, three_sum_closest(&[-1, 2, 1, -4], 1));
    println!("Closest sum for array {:?} and target {} is {}", [-1, 2, 1, -4], 2, three_sum_closest(&[-1, 2, 1, -4], 2));
    println!("Closest sum for array {:?} and target {} is {}", [-1, 2, 1, -4], 3, three_sum_closest(&[-1, 2, 1, -4], 3));
}
