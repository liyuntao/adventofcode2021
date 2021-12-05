use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::str::Chars;

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
    let reducer = |mut sum_arr: [usize; 12], chars: Chars| {
        chars.enumerate()
            .filter(|(_, char)| *char == '1')
            .for_each(|(index, _)| sum_arr[index] += 1);
        sum_arr
    };

    let chars = inputs.iter().map(|str| str.chars())
        .fold([0usize; 12], reducer);
    let gramma_rate_str: String = chars.iter()
        .map(|&cnt| if cnt > half_row_count { '1' } else { '0' })
        .collect();
    let gamma_rate = isize::from_str_radix(&gramma_rate_str, 2).unwrap();
    println!("q1={}", gamma_rate * (0b111111111111 ^ gamma_rate));


}
