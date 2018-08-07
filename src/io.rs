//! Basic IO operations
use std::{env, ffi::OsStr, fmt::Display, fs, io, path::Path, process, str::FromStr};

pub struct Config<T> {
    pub start_num: T,
    pub end_num: T,
    pub save_file: bool,
}

impl<T> Config<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    ///Parses `env::Args`, returns `Config`
    pub fn new(mut args: env::Args) -> Result<Config<T>, &'static str> {
        args.next();

        let start_num = match args.next() {
            Some(num) => match num.parse() {
                Ok(num) => num,
                Err(_e) => return Err("Cannot parse start_num"),
            },
            None => return Err("Didn't get a start_num"),
        };

        let end_num = match args.next() {
            Some(num) => match num.parse() {
                Ok(num) => num,
                Err(_e) => return Err("Cannot parse end_num"),
            },
            None => return Err("Didn't get a end_num"),
        };

        let save_file = match args.next() {
            Some(f) => f[2..].to_bool(),
            None => false,
        };

        Ok(Config {
            start_num,
            end_num,
            save_file,
        })
    }

    ///Request for `Config` arguments
    pub fn request() -> Config<T> {
        println!("Please input the start number:");
        let start_num = input_as_num();

        println!("Please input the end number:");
        let end_num = input_as_num();

        println!("Do you want to save prime numbers to a file? (yes/no)");
        let save_file = read_input().to_bool();

        Config {
            start_num,
            end_num,
            save_file,
        }
    }
}

///Reads user input and returns it as `String`
pub fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}

///Reads user input and converts it to unsigned integer (`PrimeN`)
pub fn input_as_num<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut input;
    let num: T;

    loop {
        input = read_input();

        num = match input.parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please try again. Error: {}", e);
                input.clear();
                continue;
            }
        };

        break;
    }

    num
}

pub trait ToBool<'a> {
    const TRUE_ANS: [&'a str; 9] = [
        "accept", "agree", "do it", "ok", "okay", "save", "sure", "true", "yes",
    ];

    fn as_str(&self) -> &str;

    ///Transform `&self` into `bool` value.
    ///Returns true, if value is one of the `TRUE_ANS`.
    ///
    /// # Examples
    /// ```
    /// # use prime_number::io::ToBool;
    /// assert!("yes".to_bool());
    /// assert!(!"no".to_bool());
    /// ```
    fn to_bool(&self) -> bool {
        Self::TRUE_ANS.contains(&self.as_str())
    }
}

impl<'a> ToBool<'a> for String {
    fn as_str(&self) -> &str {
        self
    }
}

impl<'a> ToBool<'a> for str {
    fn as_str(&self) -> &str {
        self
    }
}

///Creates directories and file. Writes data to file.
///This is a convenience function for using fs::create_dir_all and fs::write.
///
/// # Examples
/// ```
/// use prime_number::io;
///
/// io::write_to_dir("foo", "bar.txt", b"baz");
/// ```
pub fn write_to_dir<P, C>(dir: P, filename: P, content: C) -> Result<(), io::Error>
where
    P: AsRef<Path> + AsRef<OsStr>,
    C: AsRef<[u8]>,
{
    let file_path = Path::new(&dir).join(&filename);

    fs::create_dir_all(dir)?;
    fs::write(file_path, content)?;
    Ok(())
}

///Displays error `message` and `err` to user. Exits the program.
pub fn exit_with_err<T: Display>(err: T, message: &str) {
    eprintln!("{}: {}", message, err);
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_bool() {
        assert!("true".to_bool());
        assert!(!"no".to_bool());
    }

    #[test]
    fn string_bool() {
        assert!("true".to_string().to_bool());
        assert!(!"no".to_string().to_bool());
    }
}
