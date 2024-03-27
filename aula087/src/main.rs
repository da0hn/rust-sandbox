fn move_zeros(input_numbers: Vec<i32>) -> Vec<i32> {
    let mut output_numbers = Vec::<i32>::new();
    for n in input_numbers.iter() {
        if *n != 0 {
            output_numbers.push(*n);
        }
    }
    output_numbers.resize(input_numbers.len(), 0);
    output_numbers
}

fn main() {
    let input_numbers = vec![0, 1, 3, 0, 12];
    println!("{:?}", move_zeros(input_numbers));
}
