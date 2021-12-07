
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn parse_input() -> Vec<usize> {
    read_input("day04_input.txt").unwrap()
        .get(0).unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

fn parse_data() {

}

struct Matrix<const LEN: usize> {
    inner: [[usize; LEN]; LEN],
}

impl<const LEN: usize> Matrix<LEN> {
    pub fn new() -> Matrix<LEN> {
        return Matrix {
            inner: [[0; LEN]; LEN],
        };
    }
}

fn main() {
    let inputs:Vec<usize> = parse_input();
    println!("{:?}", inputs);
}
