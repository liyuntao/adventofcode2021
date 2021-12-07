use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<i32>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn count_total_incr(inputs: &[i32]) -> usize {
    let len = inputs.len();
    // inputs.array_windows::<2>()
    //     .filter(|&arr_win| arr_win[1] > arr_win[0])
    //     .count()
    inputs[0..len - 1].iter().zip(&inputs[1..len])
        .filter(|tuple| tuple.1 > tuple.0)
        .count()
}

fn main() {
    let numbers = read_input("day01.txt").unwrap();
    // q1: count total increases window by 1
    println!("q1={}", count_total_incr(&numbers));
    // q2: count total increases window by 3
    let win_by_3then_sum: Vec<i32> = numbers.windows(3).map(|win| win.iter().sum()).collect();
    println!("q2={}", count_total_incr(&win_by_3then_sum));
}
