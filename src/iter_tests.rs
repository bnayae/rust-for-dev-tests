
#[cfg(test)]
mod iter_tests {
    // use super::*;
    // use chrono::{DateTime, Utc};

    #[test]
    fn chars_iterator() {
        let mut chars = "abc".chars();
        assert_eq!(chars.as_str(), "abc");
        let c: Option<char> = chars.next();
        assert_eq!(c, Some('a'));
        let c: Option<char> = chars.next();
        assert_eq!(c, Some('b'));
        let c: Option<char> = chars.next();
        assert_eq!(c, Some('c'));
        let c: Option<char> = chars.next();
        assert_eq!(c, None);
    }

    #[test]
    fn chars_iterator_as_str() {
        let mut chars = "abc".chars();
        let c: Option<char> = chars.next();
        assert_eq!(c, Some('a'));
        let c: Option<char> = chars.next();
        assert_eq!(c, Some('b'));
        let c = chars.next();
        assert_eq!(c, Some('c'));
        let c = chars.next();
        assert_eq!(c, None);
    }
}
