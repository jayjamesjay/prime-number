# Prime Number
[![Build Status](https://travis-ci.org/jayjamesjay/prime-number.svg?branch=master)](https://travis-ci.org/jayjamesjay/prime-number)

Generate all prime numbers between two given numbers.

## Features
* Relatively simple
* No additional dependencies
* Limited to 64-bit integers
* Save primes to file
* CLI

## CLI Usage
```
Usage:
  prime_number [ARGS] [OPTIONS]

Arguments:
  <start_num>
  <end_num>

Options:
  --save

```

## Build from source
1. Make sure you have installed [Cargo & Rust](https://www.rust-lang.org/)
2. Download repository and extract zip or clone the repository using [Git](https://git-scm.com/):
```
$ git clone https://github.com/jayjamesjay/prime-number.git
```
3. Open repository folder in terminal
```
$ cd repository-location/repository-folder
```
4. Build using cargo
```
$ cargo build --release
```
Built program should be placed in `repository-location/repository-folder/target/release`.

## License
Licensed under [MIT](https://github.com/jayjamesjay/prime-number/blob/master/LICENSE).
