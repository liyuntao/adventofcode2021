extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input/day04.txt");

fn parse_and_prepare(s: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\D+").unwrap();
    }

    let lines: Vec<&str> = s.lines().collect();
    let numbers: Vec<usize> = lines[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let matrix: Vec<Vec<usize>> = lines.iter()
        .filter(|str| !str.contains(","))
        .filter(|str| str.len() > 5)
        .map(|line|
            RE.split(line.trim())
                .into_iter()
                .map(|part| part.parse::<usize>().unwrap())
                .collect()
        ).collect();
    (numbers, matrix)
}

fn q1(s: &str) -> usize {
    let (inputs, matrix, state_grid) = parse_and_prepare(s);
    let matrix_rows = matrix.len();
    let mut state_grid: Vec<Vec<bool>> = vec![vec![false; 5]; matrix_rows];
    inputs.iter().for_each(|&n| {
        for y in 0..matrix_rows {
            for x in 0..5 {
                if matrix[y][x] == n {
                    state_grid[y][x] = true;
                }
            }
        }
    });
    1
}

fn q2(s: &str) -> usize {
    1
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 4512);
    }

    // #[test]
    // fn q2_test() {
    //     assert_eq!(q2(INPUT), 12);
    // }
}