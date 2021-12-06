use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_last_n(inputs: Vec<usize>, total: usize, preserve_major: bool) -> usize {
    if inputs.len() == 1 {
        inputs[0]
    } else {
        let index = total - 1;
        let cur_len = inputs.len();
        let bit1_count = inputs.iter().filter(|&n| (n >> index) & 1 == 1).count();
        let major_bit = (2 * bit1_count >= cur_len) as usize;
        let preserve_bit = if preserve_major { major_bit } else { !(major_bit != 0) as usize };
        let new_inputs = inputs.into_iter()
            .filter(|&n| (n >> index) & 1 == preserve_bit)
            .collect();
        get_last_n(new_inputs, index, preserve_major)
    }
}

fn main() {
    let inputs = read_input("day03.txt").unwrap();
    const N: usize = 12;
    let half_row_count = inputs.len() / 2;
    let reducer = |mut sum_arr: [usize; N], n: &usize| -> [usize; N] {
        (0..N).for_each(|index|
            sum_arr[index] += (n >> (N - 1 - index)) & 1);
        sum_arr
    };

    let input_nums: Vec<usize> = inputs.iter()
        .map(|str| usize::from_str_radix(&str, 2).unwrap())
        .collect();
    let gamma_rate = input_nums.iter()
        .fold([0usize; N], reducer)
        .map(|cnt| (cnt > half_row_count) as usize)
        .iter().enumerate()
        .fold(0usize, |mut acc, (i, n)| {
            acc += n << (N - 1 - i);
            acc
        });
    println!("q1={}", gamma_rate * (((1 << N) - 1) ^ gamma_rate));
    println!("q2={}", get_last_n(input_nums.clone(), N, true)
        * get_last_n(input_nums, N, false));
}
