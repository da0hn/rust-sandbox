use io::stdin;
use std::io;

fn convert_to_int(input: &String) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}


fn main() {
    let mut raw_input_number = String::new();

    stdin().read_line(&mut raw_input_number)
        .expect("Failed to read line");

    let mut input_number = convert_to_int(&raw_input_number);
    let mut total = 0;
    while input_number != 0 {
        let digit = input_number % 10;
        total += digit;
        input_number = input_number / 10;
    }
    println!("The sum of the digits is: {}", total);
}
