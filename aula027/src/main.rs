fn main() {
    count(10);
    count_down(10);
}

fn count(num: i32) {
    for n in 1..=num {
        println!("{}", n);
    }
}

fn count_down(mut num: i32) {
    while num > 0 {
        println!("{}", num);
        num -= 1;
    }
}

