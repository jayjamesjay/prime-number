#[cfg(test)]
mod tests;

use std::{cmp, fs, io::{self, prelude::*}, path::Path};

pub struct PrimesGroup {
    //First number to check
    start_num: u64,
    //Last number to check
    end_num: u64,
    //Prefered way of sorting numbers (A - ascending, D - descending)
    order: char,
}

impl PrimesGroup {
    pub fn new(num_1: u64, num_2: u64) -> PrimesGroup {
        let order: char;

        if num_1 > num_2 {
            order = 'D';
        } else {
            order = 'A';
        }

        PrimesGroup {
            start_num: cmp::min(num_1, num_2),
            end_num: cmp::max(num_1, num_2),
            order,
        }
    }

    ///Generates all prime numbers in selected range from `start_num` to `end_num`
    pub fn generate_primes(&self) -> Vec<u64> {
        let mut primes = Vec::new();
        let mut primality: bool;

        //Checks every number if it's prime or not
        for num in self.start_num..(self.end_num + 1) {
            primality = true;

            match num {
                0 | 1 => primality = false,
                2 => primality = true,
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

            if primality == true {
                primes.push(num);
            }
        }

        if self.order == 'D' {
            primes.reverse();
        }

        primes
    }
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
        let file_path: String = format!("{}/{}.{}", dir_path, name, extension);

        SimpleFile {
            name,
            extension,
            file_path,
            dir_path,
        }
    }

    ///Creates directory in `dir_path`
    pub fn create_dir(&self) {
        let dir_path = Path::new(&self.dir_path);
        fs::create_dir_all(&dir_path).expect("Couldn't create a directory.");
    }

    ///Creates new empty file in `file_path`
    pub fn create_file(&self) {
        let file_path = Path::new(&self.file_path);
        fs::File::create(&file_path).expect("Couldn't create a file.");
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

///Reads user input and returns it as String
pub fn read_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    user_input.trim().to_string()
}

///Reads user input and converts it to integer (u64)
pub fn input_as_num() -> u64 {
    let mut user_input: String;
    let number: u64;

    loop {
        user_input = read_user_input();

        number = match user_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please try again.");
                user_input.clear();
                continue;
            }
        };

        break;
    }

    number
}

///Converts Vec to String
pub fn vec_to_string(vec: &Vec<u64>) -> String {
    let mut output_string = String::new();
    let mut line_start: u64 = 0;
    let mut line_end: u64;

    for x in vec {
        //Adds whitespace after every value
        let value = x.to_string() + " ";
        output_string.push_str(&value);

        line_end = output_string.len() as u64;

        //Breaks line after (at least) 75 signs
        if line_end - line_start >= 75 {
            output_string.push_str("\n");
            line_start = line_end;
        }
    }

    output_string.trim().to_string()
}
