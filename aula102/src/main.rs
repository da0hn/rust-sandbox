enum Asset {
    Stocks,
    Bonds,
    Funds,
    Cash,
}

impl Asset {
    fn price(&self) -> f64 {
        match self {
            Asset::Stocks => 10.0,
            Asset::Bonds => 20.0,
            Asset::Funds => 30.0,
            Asset::Cash => 40.0,
        }
    }
}


fn main() {
    let portfolio = [Asset::Stocks, Asset::Bonds, Asset::Funds, Asset::Cash];

    let total = portfolio.iter().map(|asset| asset.price()).sum::<f64>();
    println!("Total: {}", total);
}
