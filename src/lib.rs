fn append_s(s: &str) -> String {
    format!("{}s", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(append_s("http"), "https".to_string());
        assert_eq!(append_s("append"), "appends".to_string());
        assert_eq!(append_s("beginner"), "beginners".to_string());
    }
}
