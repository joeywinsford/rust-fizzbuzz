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

macro_rules! fizz_tests {
    ($($testname:ident: $params:expr ),*) => {
        $(
            #[test]
            fn $testname() {
                let (n, expected) = $params;
                assert_eq!(expected, to_fizzbuzz(n));
            }
        )*
    };
}

fizz_tests! {
    //val_for_1: (1, 1),
    fizz_for_3: (3, FizzBuzzResult::Fizz),
    buzz_for_5: (5, FizzBuzzResult::Buzz),
    fizz_for_6: (6, FizzBuzzResult::Fizz),
    buzz_for_10: (10, FizzBuzzResult::Buzz),
    fizzbuzz_for_15: (15, FizzBuzzResult::FizzBuzz)
}
