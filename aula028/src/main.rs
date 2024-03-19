fn main() {
    let numbers = [1, 2, 3, 113, 134, 35, 63, 141, 123, 94, 40, 51, 35, 103, 95, 6, 7, 8, 9, 10];
    println!("The greatest number is {}", greatest_number(&numbers));
}

fn greatest_number(numbers: &[i32]) -> i32 {
    let mut greatest = numbers[0];

    for n in numbers {
        if n > &greatest {
            greatest = *n;
        }
    }
    return greatest;
}

