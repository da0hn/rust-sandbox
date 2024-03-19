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
    let mut raw_student_grade_quantity = String::new();

    std::io::stdin().read_line(&mut raw_student_grade_quantity)
        .expect("Failed to read line");

    let student_grade_quantity = convert_to_int(&raw_student_grade_quantity);

    let mut counter = 0;
    let mut average_grade = 0.0;
    let mut students_at_risk_of_failing = 0;

    while counter < student_grade_quantity {
        let student_grade = read_i32();
        average_grade += (student_grade as f64) / (student_grade_quantity as f64);
        if student_grade < 6 {
            students_at_risk_of_failing += 1;
        }
        counter += 1;
    }

    println!("The average grade is: {:.2}", average_grade);
    println!("The number of students at risk of failing is: {}", students_at_risk_of_failing)
}
