use super::*;

#[test]
fn primes_group_new() {
    let _primes_group = PrimesGroup::new(0, 200);
}

#[test]
fn generate_primes() {
    let primes_group = PrimesGroup::new(0, 200);

    assert_eq!(
        primes_group.generate_primes(),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199,
        ]
    );

    let primes_group = PrimesGroup::new(150, 0);

    assert_eq!(
        primes_group.generate_primes(),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        ]
    );
}

#[test]
fn convert_vec_to_string() {
    let vec = vec![1, 2, 3, 6, 5, 4];

    assert_eq!(vec_to_string(&vec), "1 2 3 6 5 4");
}
