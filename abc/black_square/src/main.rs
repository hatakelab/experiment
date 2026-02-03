use std::*;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let num: Vec<i32> = word.split_whitespace().map(|t| t.parse().unwrap()).collect();
    let p = num[0];
    let q = num[1];

    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let num: Vec<i32> = word.split_whitespace().map(|t| t.parse().unwrap()).collect();
    let x = num[0];
    let y = num[1];

    if p <= x && x <= p + 100 && q <= y && y <= q + 100 {
        println!("Yes");
    } else {
        println!("No");
    }
}
