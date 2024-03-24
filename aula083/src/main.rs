fn find_kth_greatest_element(elements: Vec<i32>, k: i32) -> i32 {
    
    if k > elements.len() as i32 {
        panic!("k is greater than the number of elements");
    }
    
    let mut elements_copy = elements;
    elements_copy.sort();
    elements_copy[k as usize]
}

fn main() {
    let elements =  vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    println!("The {} greatest element is: {}", k, find_kth_greatest_element(elements, k));
}
