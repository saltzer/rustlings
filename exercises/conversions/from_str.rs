// from_str.rs
// This is similar to from_into.rs, but this time we'll implement `FromStr`
// and return errors instead of falling back to a default value.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object


impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(String::from("cannot covert"))
        }
        let split: Vec<&str> = s.split(',').collect();
        if split.len() != 2 {
            return Err(String::from("cannot covert"))
        }
        if split[0] == "" {
            return Err(String::from("cannot covert"))
        }
        match split[1].parse::<usize>() {
            Ok(v) => Ok(Person{name: String::from(split[0]), age: v}),
            Err(v) => Err(String::from("parse age fail"))
        }
    }
}


fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}