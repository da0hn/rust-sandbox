fn convert_to_int(input: &String) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}

fn read_i32() -> i32 {
    let mut raw_str = String::new();
    std::io::stdin().read_line(&mut raw_str)
        .expect("Failed to read line");
    return convert_to_int(&raw_str);
}


fn main() {
    
    println!("Enter a number: ");
    let input_number = read_i32();
    
    multiplication_table(input_number);
    
}

fn multiplication_table(number: i32) {
    
    for n in 0..=10 {
        println!("{} x {} = {}", number, n, number * n)
    }
    
}
