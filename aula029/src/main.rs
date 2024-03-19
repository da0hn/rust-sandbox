
fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let limit = (num as f64).sqrt() as i32;
    
    for n in 2..=limit {
        if num % n == 0 {
            println!("{} is divisible by {}", num, n);
            return false;
        }
    }
    return true;
} 

fn main() {
    let number = 107;
    let is_prime = is_prime(number);
    
    println!("The number {} is prime? {}", number, is_prime);
}
