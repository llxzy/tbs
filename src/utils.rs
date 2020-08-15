use std::str::FromStr;
use std::io::{self, BufRead};
use std::fs;

pub fn parse<T>(input: &str) -> T
where
    T: FromStr,
{
    let val: T = match input.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("argument not parseable"),
    };
    val
}

pub fn load_scores(filename: &str) -> Vec<u32> {
    let file = fs::File::open(filename).unwrap();
    let buf = io::BufReader::new(file);
    let mut output = Vec::new();
    for line in buf.lines() {
        let val = match line.unwrap().trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => panic!("wrong score format"),
        };
        output.push(val);
    }
    output
}

pub fn write_scores(scores: &Vec<u32>) {
    
}