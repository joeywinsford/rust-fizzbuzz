use std::fmt;

fn main() {
    for n in 1..21 {
        let x = to_fizzbuzz(n);
        println!("{} => {}", n, x);
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum FizzBuzzResult {
    Fizz,
    Buzz,
    FizzBuzz,
    Value(i32)
}

impl fmt::Display for FizzBuzzResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzBuzzResult::Fizz            => write!(f, "Fizz!"),
            FizzBuzzResult::Buzz            => write!(f, "Buzz!"),
            FizzBuzzResult::FizzBuzz            => write!(f, "FizzBuzz!"),
            FizzBuzzResult::Value(number)   => write!(f, "{}", number),
        }
    }
}

fn to_fizzbuzz(number: i32) -> FizzBuzzResult {
    match number {
        number if number % (3 * 5) == 0 => FizzBuzzResult::FizzBuzz,
        number if number % 3 == 0       => FizzBuzzResult::Fizz,
        number if number % 5 == 0       => FizzBuzzResult::Buzz,
        _   => FizzBuzzResult::Value(number),
    }
}

#[test]
fn multiple_of_3_fizzes() {
    assert_eq!(FizzBuzzResult::Fizz, to_fizzbuzz(3));
}
