/*
fn append_s(s: &str) -> String {
    format!("{}s", s)
}
*/

fn setsubun(s: &str) -> String {
    let s_s = s.to_string();
    let t: Vec<&str> = s_s.split_whitespace().collect();
    let n: i64 = t[0].parse().unwrap();
    let mut k: i64 = t[1].parse().unwrap();
    let mut ans: i64 = -1;
    while k > 0 {
        ans += 1;
        k -= n + ans;
    }
    format!("{}", ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(setsubun("4 43"), "6".to_string());
        assert_eq!(setsubun("100000000 100000000"), "0".to_string());
        assert_eq!(setsubun("1234 12345678"), "3886".to_string());
    }
}
