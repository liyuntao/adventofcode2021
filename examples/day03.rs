use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let inputs = read_input("day03.txt").unwrap();
    let half_row_count = inputs.len() / 2;
    let reducer = |mut sum_arr: [usize; 12], n: usize| {
        (0..12).for_each(|index|
            sum_arr[index] += (n >> (11 - index)) & 1);
        sum_arr
    };

    let gamma_rate = inputs.iter()
        .map(|str| usize::from_str_radix(&str, 2).unwrap())
        .fold([0usize; 12], reducer)
        .map(|cnt| if cnt > half_row_count { 1 } else { 0 })
        .iter().enumerate()
        .fold(0usize, |mut acc, (i, n)| {
            acc += n << (11 - i);
            acc
        });
    println!("q1={}", gamma_rate * (0b111111111111 ^ gamma_rate));


}
