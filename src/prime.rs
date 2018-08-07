//! Prime numbers generation 
use std::cmp::{self, Ordering};

pub struct Primes {
    //First number to check
    start_num: u64,
    //Last number to check
    end_num: u64,
    //Prefered way of sorting numbers (A - ascending, D - descending)
    order: char,
}

impl Primes {
    ///Creates new group of numbers to check for prime numbers
    ///
    /// # Examples
    /// ```
    /// use prime_number::prime::Primes;
    ///
    /// let primes_group = Primes::new(0, 100); //ok, will oreder generated primes ascendingly
    /// let primes_group = Primes::new(100, 0); //also fine, but will oreder generated primes descendingly
    /// ```
    pub fn new(num_1: u64, num_2: u64) -> Primes {
        Primes {
            start_num: cmp::min(num_1, num_2),
            end_num: cmp::max(num_1, num_2),
            order: match num_2.cmp(&num_1) {
                Ordering::Less => 'D',
                _ => 'A',
            },
        }
    }

    ///Generates all prime numbers in selected range from `start_num` to `end_num`.
    ///Sorts generated numbers according to `order`.
    ///
    /// # Examples
    /// ```
    /// use prime_number::prime::Primes;
    ///
    /// let primes_group = Primes::new(22, 55);
    /// let primes = primes_group.generate();
    ///
    /// assert_eq!(vec![23, 29, 31, 37, 41, 43, 47, 53], primes);
    /// ```
    pub fn generate(&self) -> Vec<u64> {
        let mut primes: Vec<_> = (self.start_num..=self.end_num)
            .filter(|&num| is_prime(num))
            .collect();

        if self.order == 'D' {
            primes.reverse();
        }

        primes
    }
}

///Checks primality of number
///
/// # Examples
/// ```
/// use prime_number::prime;
///
/// assert!(!prime::is_prime(134));
/// assert!(prime::is_prime(67));
/// ```
pub fn is_prime(num: u64) -> bool {
    match num {
        0 | 1 => false,
        2 => true,
        _ => division_test(num),
    }
}

///Checks if number doesn't have more than 2 dividers (itself and 1). This test applies
///to numbers higher than 2.
///
/// # Examples
/// ```
/// use prime_number::prime;
///
/// assert!(!prime::division_test(22));
/// assert!(prime::division_test(23));
/// ```
pub fn division_test(num: u64) -> bool {
    match num % 2 {
        0 => false,
        _ => {
            let mut i = 3;

            while i < (num as f64).sqrt() as u64 + 2 {
                if num % i == 0 {
                    return false;
                }

                i += 2;
            }

            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_new() {
        let _primes_group = Primes::new(0, 200);
        let _primes_group = Primes::new(220, 10);
    }

    #[test]
    fn check_prime() {
        assert!(!is_prime(22));
        assert!(is_prime(331));
    }

    #[test]
    fn generate_primes() {
        let primes_group = Primes::new(0, 200);

        assert_eq!(
            primes_group.generate(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199,
            ]
        );

        let primes_group = Primes::new(150, 0);

        assert_eq!(
            primes_group.generate(),
            vec![
                149, 139, 137, 131, 127, 113, 109, 107, 103, 101, 97, 89, 83, 79, 73, 71, 67, 61,
                59, 53, 47, 43, 41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2,
            ]
        );
    }
}
