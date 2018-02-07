extern crate prime_numbers_generator;

use prime_numbers_generator::*;

fn main() {
    println!("Welcome in Prime Numbers Generator!");

    loop {
        println!("Please input the start number:");
        let start_num = input_as_number();

        println!("Please input the end number:");
        let end_num = input_as_number();

        let primes_group = PrimeNumbersGroup::new(start_num, end_num);
        let primes = primes_group.generate_primes();

        let primes_output = vec_to_string(&primes);
        println!("Prime numbers in selected range are:\n{}", primes_output);

        println!("Do you want to save prime numbers to a file? (y/n)");
        if read_user_input() == "y" {
            let file_name = format!("prime_numbers_{}_{}", start_num, end_num);

            let file = File::new(
                file_name,
                "txt".to_string(),
                "generated_numbers".to_string(),
            );

            file.create_dir();

            let file_path = file.create_file();
            file.write_file(&file_path, &primes_output);
        }
    }
}
