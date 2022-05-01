// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::ops::Not;

pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        StrSplit {
            remainder: haystack.is_empty().not().then(|| haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)>;
}

impl Delimiter for char {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(*self).map(|i| (i, i + self.len_utf8()))
    }
}

impl Delimiter for &str {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(self).map(|i| (i, i + self.len()))
    }
}

impl<'a, 'b, D: Delimiter> Iterator for StrSplit<'a, D> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let head = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(head)
        } else {
            self.remainder.take()
        }
    }
}

pub fn until_char(haystack: &str, delimiter: char) -> Option<&str> {
    StrSplit::new(haystack, delimiter).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_split() {
        let haystack = "hello world";
        let mut iter = StrSplit::new(haystack, " ");
        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_longer_string() {
        let haystack = "1 2 3 4 5";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        let expected = vec!["1", "2", "3", "4", "5"];
        assert_eq!(letters, expected);
    }

    #[test]
    fn test_string_with_tail() {
        let haystack = "1 2 3 4 ";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        let expected = vec!["1", "2", "3", "4", ""];
        assert_eq!(letters, expected);
    }

    #[test]
    fn test_empty() {
        let haystack = "";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        let expected: Vec<&str> = vec![];
        assert_eq!(letters, expected);
    }

    #[test]
    fn test_single_delimiter() {
        let haystack = " ";
        let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
        let expected: Vec<&str> = vec!["", ""];
        assert_eq!(letters, expected);
    }

    #[test]
    fn test_until_char() {
        let haystack = "hello world";
        let letters = until_char(haystack, ' ');
        assert_eq!(letters, Some("hello"));
    }

    #[test]
    fn test_until_ut8_char() {
        let haystack = "helloßworld";
        let letters = until_char(haystack, 'ß');
        assert_eq!(letters, Some("hello"));
    }
}
