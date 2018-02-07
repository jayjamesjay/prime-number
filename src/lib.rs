use std::{cmp, fs, io, path};
use std::io::prelude::*;

pub struct PrimeNumbersGroup {
    //First number to check
    start_num: u64,
    //Last number to check
    end_num: u64,
}

impl PrimeNumbersGroup {
    pub fn new(num_1: u64, num_2: u64) -> PrimeNumbersGroup {
        PrimeNumbersGroup {
            //Chooses as 'start_num' the smallest of 2 numbers
            start_num: cmp::min(num_1, num_2),
            //Chooses as 'end_num' the largest of 2 numbers
            end_num: cmp::max(num_1, num_2),
        }
    }

    //Generates prime numbers
    pub fn generate_primes(&self) -> Vec<u64> {
        let mut numbers: Vec<u64> = Vec::new();
        let mut primality: bool;

        //Checks every number if it's prime or not
        for x in self.start_num..(self.end_num + 1) {
            primality = true;

            match x {
                //For 0 and one returns false
                0 | 1 => {
                    primality = false;
                }
                //Divides numbers larger than one by theirs 'square root + 1'
                //=> number / (square_root(number) + 1)
                _ => for i in 2..((x as f64).sqrt() as u64 + 1) {
                    if x % i == 0 {
                        primality = false;
                        break;
                    }
                },
            }

            //If number is prime, wiil be pushed to vector
            if primality == true {
                numbers.push(x);
            }
        }

        numbers
    }
}

pub struct File {
    //Filename ex. "primes"
    name: String,
    //File extension ex. "txt"
    extension: String,
    //Directory, where file will be saved ex. "generated_numbers"
    path: String,
}

impl File {
    pub fn new(name: String, extension: String, path: String) -> File {
        File {
            name,
            extension,
            path,
        }
    }

    //Creates directory ('path')
    pub fn create_dir(&self) {
        let dir_path = path::Path::new(&self.path);
        fs::create_dir_all(&dir_path).expect("Couldn't create directory.");
    }

    //Creates new empty file ('path/name.extension')
    pub fn create_file(&self) -> String {
        let file_path = String::new() + &self.path + "/" + &self.name + "." + &self.extension;
        fs::File::create(&file_path).expect("Couldn't create a file.");

        //Returns path of created file (file_path == 'path/name.extension')
        file_path
    }

    //Saves 'data' to file ('path/name.extension')
    pub fn write_file(&self, file_path: &str, data: &str) {
        //Opens file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&file_path)
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

//Reads user input and returns String
pub fn read_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    user_input.trim().to_string()
}

//Reads user input and converts it to number
pub fn input_as_number() -> u64 {
    let mut user_input: String;
    let number: u64;

    loop {
        //Reads user input using 'read_user_input'
        user_input = read_user_input();

        //Converts to num
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

//Converts vector to string
pub fn vec_to_string(vec: &Vec<u64>) -> String {
    let mut output_string = String::new();
    let mut start_point: u64 = 0;
    let mut end_point: u64;

    for x in vec {
        //Adds whitespace after every value
        let value = x.to_string() + " ";
        output_string.push_str(&value);

        end_point = output_string.len() as u64;

        //Breaks line after (at least) 76 signs
        if end_point - start_point >= 76 {
            output_string.push_str("\n");
            start_point = end_point;
        }
    }

    output_string
}
