fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();
    std::io::stdin().read_line(&mut input2).unwrap();
    let num1: Vec<i32> = input1.split_whitespace().map(|t| t.parse().unwrap()).collect();
    let num2: Vec<i32> = input2.split_whitespace().map(|t| t.parse().unwrap()).collect();
    let p = num1[0];
    let q = num1[1];
    let x = num2[0];
    let y = num2[1];

    if p <= x && x <= p + 100 && q <= y && y <= q + 100 {
        println!("Yes");
    } else {
        println!("No");
    }
}
