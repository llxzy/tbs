use std::str::FromStr;

pub fn parse<T>(input: &String) -> T
where
    T: FromStr,
{
    let val: T = match input.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("argument not parseable"),
    };
    val
}
