use std::fmt::Debug;
use std::str::FromStr;

fn convert_to<T: FromStr>(input: &String) -> T where <T as FromStr>::Err: Debug {
    return input.trim().parse::<T>()
        .expect("Failed to convert to float 64bits");
}

fn read_f64() -> f64 {
    let mut raw_str = String::new();
    std::io::stdin().read_line(&mut raw_str)
        .expect("Failed to read line");
    return convert_to(&raw_str);
}

fn read_i32() -> i32 {
    let mut raw_str = String::new();
    std::io::stdin().read_line(&mut raw_str)
        .expect("Failed to read line");
    return convert_to(&raw_str);
}


fn main() {
    let user_input_sequence_size = read_i32();

    let mut inputs = Vec::<f64>::new();
    for counter in 1..=user_input_sequence_size {
        println!("Enter the {} number: ", counter);
        let input_number = read_f64();

        let input_number_int_part = (input_number as i32) as f64;

        if (input_number_int_part - input_number) == 0.0 && (input_number as i32) % 2 == 0 {
            inputs.push(input_number);
        }
    }

    let total = inputs.iter().sum::<f64>();
    println!("The sum of the even numbers is: {}", total)
}
