use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(name: &str) -> Vec<String> {
    let path = format!("src/inputs/{name}.txt");
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}