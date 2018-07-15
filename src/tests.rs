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
        primes_group.generate_primes(),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199,
        ]
    );

    let primes_group = Primes::new(150, 0);

    assert_eq!(
        primes_group.generate_primes(),
        vec![
            149, 139, 137, 131, 127, 113, 109, 107, 103, 101, 97, 89, 83, 79, 73, 71, 67, 61, 59,
            53, 47, 43, 41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2,
        ]
    );
}

#[test]
fn convert_slice_to_string() {
    const NUMS: [u64; 6] = [1, 2, 3, 6, 5, 4];

    assert_eq!(slice_to_string(&NUMS), "1 2 3 6 5 4");
}
