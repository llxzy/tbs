use std::fs;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

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

pub fn write_scores(score: u32, filename: &str) -> io::Result<()> {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write_fmt(format_args!("{}\n", score))?;
    Ok(())
}
