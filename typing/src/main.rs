fn main() {
    let sentences: [&str; 2] = [
        "明けましておめでとうございます",
        "今年もよろしくお願いします",
    ];
    for s in sentences {
        println!("{}", s);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input.trim_end() == s {
            println!("ok");
        } else {
            println!("err");
        }
    }
}
