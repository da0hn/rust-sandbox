fn convert_to_int(input: &String) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}

fn read_u32() -> u32 {
    let mut raw_str = String::new();
    std::io::stdin().read_line(&mut raw_str)
        .expect("Failed to read line");
    return convert_to_int(&raw_str).unsigned_abs();
}


fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return greatest_common_divisor(b, a % b);
}

fn main() {
    let first_input_number = read_u32();
    let second_input_number = read_u32();

    let gcd_number = greatest_common_divisor(first_input_number, second_input_number);

    println!("The greatest common divisor of {} and {} is: {}", first_input_number, second_input_number, gcd_number);
}
