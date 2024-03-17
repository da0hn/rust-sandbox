use std::io::stdin;

fn convert_to_int(input: &String) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}

fn fatorial(n: i32) -> i32 {
    if n <= 1 { return 1; }
    return n * fatorial(n - 1);
}

fn main() {
    let mut raw_input_number = String::new();

    stdin().read_line(&mut raw_input_number)
        .expect("Failed to read line");

    let input_number = convert_to_int(&raw_input_number);

    let result = fatorial(input_number);
    println!("The fatorial of {} is: {}", input_number, result);
}
