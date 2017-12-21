extern crate prime_number;

use prime_number::*;

fn main() {
    println!("Welcome in Prime Numbers Generator!");

    loop {
        println!("Please input the start number:");
        let start_num = input_as_number();

        println!("Please input the end number:");
        let end_num = input_as_number();

        let primes_group = PrimeNumbersGroup::new(start_num, end_num);
        let primes = primes_group.generate_primes();

        let primes_output = vector_to_string(&primes);
        println!("Prime numbers in selected range are: {}", primes_output);
    }
}
