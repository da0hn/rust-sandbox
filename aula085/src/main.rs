fn maximum_profit(values: &Vec<i32>) -> i32 {
    let mut min_price = values[0];
    let mut min_price_index = 0;
    for idx in 1..values.len() {
        if values[idx] < min_price {
            min_price = values[idx];
            min_price_index = idx;
        }
    }
    if min_price_index == values.len() - 1 {
        return 0;
    }
    let mut max_price = 0;
    for idx in min_price_index..values.len() {
        if values[idx] > max_price {
            max_price = values[idx];
        }
    }
    max_price - min_price
}


fn main() {
    let values = vec![7, 1, 5, 3, 6, 4];
    println!("for values {:?} the maximum budget is: {}", values, maximum_profit(&values));
    let values = vec![7, 6, 4, 3, 1];
    println!("for values {:?} the maximum budget is: {}", values, maximum_profit(&values));
}
