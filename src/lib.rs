#[cfg(test)]
mod iter_tests {
    // use super::*;
    use std::slice::Iter;

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
        assert_eq!(chars.as_str(), "abc");
        let c: Option<char> = chars.next();
        assert_eq!(chars.as_str(), "bc");
        chars.next();
        assert_eq!(chars.as_str(), "c");
        chars.next();
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn nth() {
        let a = [1, 2, 3];
        let mut iter: Iter<i32> = a.iter();
        let res: Option<&i32> = iter.nth(1);
        assert_eq!(res, Some(&2));
        let res: Option<&i32> = iter.nth(4);
        assert_eq!(res, None);
    }
}
