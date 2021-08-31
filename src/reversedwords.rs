#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
      assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}
pub fn reverse_words(str:&str) -> String {
    str.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}