fn append_s(s: &str) -> &str {
    &format!("{}s", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(append_s("http"), "https");
        assert_eq!(append_s("append"), "appends");
        assert_eq!(append_s("beginner"), "beginners");
    }
}
