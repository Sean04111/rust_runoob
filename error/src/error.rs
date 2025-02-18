use std::{
    fmt::{self, write},
    num::ParseIntError,
};

pub fn error_basic() {
    let result: Result<i32, String> = Ok(32);
    let value = result.unwrap();
    println!("value is {}", value);

    let numbers = vec![1, 3, 5];
    match find_first_even(numbers) {
        Some(num) => println!("found {}", num),
        None => println!("not found"),
    }

    match parse_int("123") {
        Ok(value) => println!("the value is {}", value),
        Err(_) => println!("invalid string"),
    }

    match myerror_test() {
        Ok(_) => println!("ok"),
        Err(err) => println!("Error:{}", err),
    }
}

// ? mark can only be used in funciton with Option or Result type return
// if catch None or Err, the function will return None or Err

fn find_first_even(nums: Vec<i32>) -> Option<i32> {
    let first_even = nums.iter().find(|&number| number % 2 == 0)?;
    println!("Option");
    Some(*first_even)
}

fn parse_int(number: &str) -> Result<i32, ParseIntError> {
    let value = number.parse::<i32>()?;
    Ok(value)
}

#[derive(Debug)]
struct Myerror {
    detail: String,
}

impl std::fmt::Display for Myerror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my error: {}", self.detail)
    }
}

impl std::error::Error for Myerror {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn myerror_test() -> Result<(), Myerror> {
    Err(Myerror {
        detail: "this is my error".to_owned(),
    })
}
