#[allow(dead_code)]
pub fn methods(x: i32, y: i32) {
    use std::ops::{Add, Sub, Mul, Div, Rem};
    println!("{} + {} = {}", x, y, x.add(y));
    println!("{} - {} = {}", x, y, x.sub(y));
    println!("{} * {} = {}", x, y, x.mul(y));
    println!("{} / {} = {}", x, y, x.div(y));
    println!("{} % {} = {}", x, y, x.rem(y));
}