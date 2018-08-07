//! Module for working with iterators
use std::vec;

pub trait Stringify {
    fn stringify(self, separator: &str, line_len: usize) -> String;
}

impl<T: ToString> Stringify for vec::IntoIter<T> {
    ///Consumes iterator over `Vec`. Generates simply formated `String`.
    ///Generated `String` contains every value of slice separated by `separator`.
    ///It's also divided into lines with chosen length (`line_len`).
    ///
    /// # Examples
    /// ```
    /// use prime_number::iter::Stringify;
    ///
    /// let nums = vec![2, 33, 4, 55, 100];
    /// let nums_str = nums.into_iter().stringify(" | ", 80);
    ///
    /// assert_eq!("2 | 33 | 4 | 55 | 100", nums_str);
    /// ```
    fn stringify(self, separator: &str, line_len: usize) -> String {
        let mut results = String::new();
        let mut current_len = 0;

        for val in self {
            let val = val.to_string() + separator;
            current_len += val.len();

            if current_len > line_len {
                results.push_str("\r\n");
                current_len = val.len();
            }

            results.push_str(&val);
        }

        results[..(results.len() - separator.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_stringify() {
        let nums = vec![1, 2, 3, 6, 5, 4];

        assert_eq!(nums.clone().into_iter().stringify(" ", 80), "1 2 3 6 5 4");
        assert_eq!(
            nums.clone().into_iter().stringify(" | ", 80),
            "1 | 2 | 3 | 6 | 5 | 4"
        );
        assert_eq!(
            nums.clone().into_iter().stringify(" 4 ", 80),
            "1 4 2 4 3 4 6 4 5 4 4"
        );
        assert_eq!(nums.into_iter().stringify("", 80), "123654");
    }
}
