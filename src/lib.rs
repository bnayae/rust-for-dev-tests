#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn chars_iterator() {
        let mut chars = "abc".chars();
        assert_eq!(chars.as_str(), "abc");
        chars.next();
        assert_eq!(chars.as_str(), "bc");
        chars.next();
        assert_eq!(chars.as_str(), "c");
        chars.next();
        assert_eq!(chars.as_str(), "");        
    }
}
