extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input/day04.txt");

fn parse_input(s: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\D+").unwrap();
    }

    let lines: Vec<&str> = s.lines().collect();
    let numbers: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    let matrix: Vec<Vec<usize>> = lines
        .iter()
        .filter(|str| !str.contains(","))
        .filter(|str| str.len() > 5)
        .map(|line| {
            RE.split(line.trim())
                .into_iter()
                .map(|part| part.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (numbers, matrix)
}

fn q1(s: &str) -> usize {
    inner(s, true)
}
fn q2(s: &str) -> usize {
    inner(s, false)
}

fn inner(s: &str, peek_first: bool) -> usize {
    let (inputs, matrix) = parse_input(s);
    let matrix_rows = matrix.len();
    let mut state_grid: Vec<Vec<bool>> = vec![vec![false; 5]; matrix_rows];
    let mut state_array: Vec<bool> = vec![false; matrix_rows / 5];

    for n in inputs {
        for y in 0..matrix_rows {
            for x in 0..5 {
                if matrix[y][x] == n {
                    state_grid[y][x] = true;
                }
                // do complete-row checking
                let mut is_all_set = state_grid[y].iter().all(|&mark| mark == true);
                {
                    // do complete-col checking
                    let row_start = (y / 5) * 5;
                    let row_end = row_start + 5;
                    is_all_set = is_all_set
                        || (row_start..row_end)
                            .into_iter()
                            .all(|y| state_grid[y][x] == true);
                }

                if peek_first && is_all_set {
                    // using (x, y) seek for a 5*5
                    return n * sum_of_unmark_part(y, &matrix, &state_grid);
                } else if !peek_first && is_all_set {
                    // peek last succ
                    state_array[y / 5] = true;
                    if state_array.iter().all(|&v| v == true) {
                        return n * sum_of_unmark_part(y, &matrix, &state_grid);
                    }
                }
            }
        }
    }
    0
}

fn sum_of_unmark_part(row: usize, matrix: &Vec<Vec<usize>>, state_grid: &Vec<Vec<bool>>) -> usize {
    let row_start = (row / 5) * 5;
    let row_end = row_start + 5;
    let mut unmark_sum = 0;
    for y in row_start..row_end {
        for x in 0..5 {
            if state_grid[y][x] == false {
                unmark_sum += matrix[y][x];
            }
        }
    }
    return unmark_sum;
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

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 1924);
    }
}
