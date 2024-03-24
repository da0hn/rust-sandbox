#[warn(dead_code)]
enum Payment {
    Money(f64),
    DebitCard(bool, f64),
    CreditCard(bool, f64),
}


fn main() {
    let payment_type = Payment::DebitCard(true, 100.0);

    match payment_type {
        Payment::Money(amount) => println!("Payment with money: R$ {:.2}", amount),
        Payment::DebitCard(true, amount) => println!("Payment with debit card: R$ {:.2}", amount),
        Payment::CreditCard(true, amount) => println!("Payment with Credit Card: R$ {:.2}", amount),
        Payment::CreditCard(false, _) => println!("Payment with Credit Card failed"),
        _ => println!("Unknown payment type"),
    }
}
