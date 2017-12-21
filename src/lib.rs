use std::io;

pub struct PrimeNumbersGroup {
    start_num: u64,
    end_num: u64,
}

impl PrimeNumbersGroup {
    pub fn new(start_num: u64, end_num: u64) -> PrimeNumbersGroup {
        PrimeNumbersGroup { start_num, end_num }
    }

    pub fn generate_primes(&self) -> Vec<u64> {
        let mut numbers: Vec<u64> = Vec::new();
        let mut primality: bool;

        for x in self.start_num..self.end_num {
            primality = true;

            for i in 2..(((x + 1) as f64).sqrt().ceil() as u64) {
                if x % i == 0 {
                    primality = false;
                    break;
                }
            }

            if primality == true {
                numbers.push(x);
            }
        }

        numbers
    }
}

pub fn input_as_number() -> u64 {
    let mut user_input = String::new();
    let number: u64;

    loop {
        io::stdin().read_line(&mut user_input).expect(
            "Failed to read line",
        );

        number = match user_input.trim().parse() {
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

pub fn vector_to_string(vec: &Vec<u64>) -> String {
    let mut output_string = String::new();

    for x in vec {
        let value = &mut x.to_string();
        value.push_str(" ");
        output_string.push_str(value);
    }

    output_string
}
