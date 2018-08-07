extern crate prime_number;

use prime_number::{
    io::{self, Config},
    iter::Stringify,
    prime::Primes,
};
use std::{env, process, io::Error};

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

    if let Err(e) = run(config) {
        io::exit_with_err(e, "Problem with saving file")
    }
}

fn run(config: Config<u64>) -> Result<(), Box<Error>> {
    println!("Generating prime numbers...");

    let primes_group = Primes::new(config.start_num, config.end_num);
    let primes = primes_group.generate();

    if !primes.is_empty() {
        let primes = primes.into_iter().stringify(" ", 80);
        println!("Prime numbers in selected range are:\r\n{}", primes);

        if config.save_file {
            const PREFIX: &str = "prime_numbers";
            let filename = format!("{}_{}_{}.txt", PREFIX, config.start_num, config.end_num);

            io::write_to_dir(PREFIX, &filename, &primes.as_bytes())?;
            println!("File {} have been saved.", filename);
        }
    } else {
        println!("There aren't any prime numbers in selected range.");
    }

    Ok(())
}
