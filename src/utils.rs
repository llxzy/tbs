use std::str::FromStr;

pub fn parse<T: FromStr>(input: &String) -> T {
    let val: T = match input.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("argument not parseable")
    };
    val
}