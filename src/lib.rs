//Copyright (c) 2018 jayjamesjay. Licensed under MIT
//(https://github.com/jayjamesjay/prime-number/blob/master/LICENSE).
#[cfg(test)]
mod tests;

use std::{
    cmp::{self, Ordering}, fs, io::{self, prelude::*},
};

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

    ///Generates all prime numbers in selected range from `start_num` to `end_num`
    pub fn generate_primes(&self) -> Vec<u64> {
        let mut primes = Vec::new();

        //Checks primality of every number in the `Primes`
        for num in self.start_num..=self.end_num {
            if is_prime(num) {
                primes.push(num);
            }
        }

        if self.order == 'D' {
            primes.reverse();
        }

        primes
    }
}

///Checks primality of single number
pub fn is_prime(num: u64) -> bool {
    let mut primality = true;

    match num {
        0 | 1 => primality = false,
        2 => {}
        _ => match num % 2 {
            0 => primality = false,
            _ => {
                let mut i = 3;

                while i < (num as f64).sqrt() as u64 + 2 {
                    if num % i == 0 {
                        primality = false;
                        break;
                    }

                    i += 2;
                }
            }
        },
    }

    primality
}

pub struct SimpleFile {
    //Name of the file
    name: String,
    //File's extension
    extension: String,
    //Path of the directory, in which file will be saved.
    dir_path: String,
    //Direct path to the file.
    file_path: String,
}

impl SimpleFile {
    pub fn new(name: String, extension: String, dir_path: String) -> SimpleFile {
        let file_path = format!("{}/{}.{}", dir_path, name, extension);

        SimpleFile {
            name,
            extension,
            file_path,
            dir_path,
        }
    }

    ///Creates directory in `dir_path`
    pub fn create_dir(&self) {
        fs::create_dir_all(&self.dir_path).expect("Couldn't create a directory.");
    }

    ///Creates new empty file in `file_path`
    pub fn create_file(&self) {
        fs::File::create(&self.file_path).expect("Couldn't create a file.");
    }

    ///Saves data to file
    pub fn write_file(&self, data: &str) {
        //Opens file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .expect("Couldn't open the file.");

        //Writes data
        file.write_all(data.as_bytes())
            .expect("Couldn't write the file.");

        //Displays information about saving the file
        println!(
            "File \"{}.{}\" has been saved.",
            &self.name, &self.extension
        );
    }
}

///Reads user input and returns it as `String`
pub fn read_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    user_input.trim().to_string()
}

///Reads user input and converts it to unsigned integer (`u64`)
pub fn input_as_num() -> u64 {
    let mut user_input: String;
    let number: u64;

    loop {
        user_input = read_user_input();

        number = match user_input.parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please try again. Error: {}", e);
                user_input.clear();
                continue;
            }
        };

        break;
    }

    number
}

///Converts slice (`&[u64]`) to `String`
pub fn slice_to_string(slice: &[u64]) -> String {
    let mut output_string = String::new();
    let mut line_length = 0;

    for val in slice {
        //Adds whitespace after every value
        let val = val.to_string() + " ";
        output_string.push_str(&val);

        line_length += val.len() as u8;

        //Breaks line after (at least) 75 signs
        if line_length >= 75 {
            output_string.push_str("\r\n");
            line_length = 0;
        }
    }

    output_string.trim().to_string()
}
