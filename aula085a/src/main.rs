fn maximum_profit(values: Vec<i32>) -> i32 {
    if values.is_empty() {
        return 0;
    }

    let mut min_price = values[0];
    let mut max_profit = 0;

    for price in values.iter().skip(1) {
        let current_profit = price - min_price;
        max_profit = max_profit.max(current_profit);
        min_price = min_price.min(*price);
    }
    max_profit
}


fn main() {
    let values = vec![7, 1, 5, 3, 6, 4];
    println!("for values {:?} the maximum budget is: {}", values, maximum_profit(values.clone()));
    let values = vec![7, 6, 4, 3, 1];
    println!("for values {:?} the maximum budget is: {}", values, maximum_profit(values.clone()));
}
