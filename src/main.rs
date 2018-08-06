extern crate prime_number;

use prime_number::{
    io::{self, Config, FFile},
    iter::Stringify,
    prime::Primes,
};
use std::{env, process};

fn main() {
    let args = env::args();
    println!("Welcome in Prime Numbers Generator!");

    let config = match args.len() {
        3 | 4 => Config::new(args).unwrap_or_else(|e| {
            eprintln!("Problem with parsing arguments: {}", e);
            process::exit(1);
        }),
        _ => Config::request(),
    };

    println!("Generating prime numbers...");

    let primes_group = Primes::new(config.start_num, config.end_num);
    let primes = primes_group.generate();

    if !primes.is_empty() {
        let primes = primes.into_iter().stringify(" ", 80);
        println!("Prime numbers in selected range are:\r\n{}", primes);

        if config.save_file {
            let file_name = format!("prime_numbers_{}_{}.txt", config.start_num, config.end_num);
            let file = FFile::new(&file_name, "prime_numbers");

            file.create_dirs()
                .unwrap_or_else(|e| io::close_with_err(e, "Problem with creating directories"));
            file.save(&primes)
                .unwrap_or_else(|e| io::close_with_err(e, "Problem with saving file"));

            println!("File {} have been saved.", file_name);
        }
    } else {
        println!("There aren't any prime numbers in selected range.");
    }
}
