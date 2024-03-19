use std::fmt::Debug;
use std::str::FromStr;

fn convert_to<T: FromStr>(input: &String) -> T where <T as FromStr>::Err: Debug {
    return input.trim().parse::<T>()
        .expect(("Failed to convert to ".to_owned() + std::any::type_name::<T>()).as_str());
}

fn read_number<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    let mut raw_str = String::new();
    std::io::stdin().read_line(&mut raw_str)
        .expect("Failed to read line");
    return convert_to::<T>(&raw_str);
}

fn calculate_average(grades: &[f64]) -> f64 {
    if grades.len() == 0 {
        return 0.0;
    }
    let mut total = 0.0;
    for grade in grades {
        total += grade
    }
    return total / grades.len() as f64;
}

fn main() {
    let grades = [7.5, 8.0, 9.0, 6.5, 7.0, 8.5, 9.5, 6.0, 7.0, 8.0];
    let average_grade = calculate_average(&grades);
    println!("The average grade is: {:.2}", average_grade);
}
