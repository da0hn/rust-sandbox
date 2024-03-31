struct Coin {
    value: u32,
}

impl Coin {
    fn new(value: u32) -> Self {
        Coin { value }
    }

    fn get_value(&self) -> u32 { self.value }

    fn set_value(&mut self, value: u32) { self.value = value; }
}




fn main() {
    
    let mut coin = Coin::new(10);
    println!("Valor da moeda: {}", coin.get_value());
    coin.set_value(20);
    println!("Valor da moeda: {}", coin.get_value());
}
