fn method1() {
    let x = 10;
    let y = &x;
    let z = &x;

    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn increment(x: &mut i32) {
    *x += 1;
}

fn method3() {
    let mut x = 5;
    let z = &mut x;
    increment(z);
    println!("z: {}", z);
}

fn method2() {
    let mut x = 10;
    let y = &mut x;

    *y += 1;

    println!("y: {}", y);
}

fn update_value(x: &i32, y: &mut i32) {
    *y = *x + 10;
}

fn method4() {
    let a = 5;
    let mut b = 0;
    let c = &a;
    update_value(c, &mut b);
    println!("b: {}", b);
}

fn method5() {
    let x = 5;
    let y = &x;
    let z = &y;
    let a = **z;
    println!("a: {}", a);
}

struct Point {
    x: i32,
    y: i32,
}

fn update_x(point: &mut Point) {
    point.x += 1;
}

fn method6() {
    let mut p1 = Point { x: 1, y: 2 };
    let p2 = &mut p1;
    let p3 = &mut *p2;
    update_x(p3);
    println!("p2.x: {}", p2.x);
}


fn main() {
    method1();
    method2();
    method3();
    method4();
    method5();
    method6();
}
