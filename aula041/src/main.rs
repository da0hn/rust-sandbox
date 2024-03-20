fn is_palindrome(number: i32) -> bool {
    if number < 0 || (number % 10 == 0 && number != 0) {
        return false;
    }

    let mut current_number = number;
    let mut reverse_number = 0;
    while current_number != 0 {
        let digit = current_number % 10;
        reverse_number = reverse_number * 10 + digit;
        current_number /= 10;
    }
    number == reverse_number
}

fn main() {
    println!("121 is palindrome? {}", is_palindrome(121));
    println!("1001 is palindrome? {}", is_palindrome(1001));
    println!("-121 is palindrome? {}", is_palindrome(-121));
    println!("10 is palindrome? {}", is_palindrome(10));
}
