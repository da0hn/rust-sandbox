fn rotate_array(array: &mut [i32], k: usize) {
    let array_size = array.len();

    if array_size == 0 { return; }

    let mut rot_count = 0usize;
    while rot_count < k {
        let last_value = array[array_size - 1];
        for idx in (0..array_size).rev() {
            if idx == 0 {
                array[idx] = last_value;
                continue;
            }
            array[idx] = array[idx-1]; 
        }
        rot_count += 1;
    }
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7];
    println!("original: {:?}", array);
    
    rotate_array(&mut array, 3);
    println!("k: 3, array: {:?}", array);
    array = [1, 2, 3, 4, 5, 6, 7];

    rotate_array(&mut array, 4);
    println!("k: 4, array: {:?}", array);
    array = [1, 2, 3, 4, 5, 6, 7];

    rotate_array(&mut array, 5);
    println!("k: 5, array: {:?}", array);
    array = [1, 2, 3, 4, 5, 6, 7];
    
    rotate_array(&mut array, 6);
    println!("k: 6, array: {:?}", array);
    array = [1, 2, 3, 4, 5, 6, 7];

    rotate_array(&mut array, 10);
    println!("k: 10, array: {:?}", array);
}
