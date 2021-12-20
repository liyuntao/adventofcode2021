extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input/day05.txt");


fn parse(line: &str) -> (usize, usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\D+").unwrap();
    }
    // 7,0 -> 7,4  =>  x1 y1 -> x2 y2
    let split: Vec<usize> = RE.split(line).into_iter()
        .map(|str| str.parse::<usize>().unwrap())
        .collect();
    (split[0], split[2], split[1], split[3])
}

fn q1(s: &str) -> usize {
    inner(s, false)
}

fn q2(s: &str) -> usize {
    inner(s, true)
}

fn smart_range_inclusive(a: usize, b: usize) -> Box<dyn Iterator<Item=usize>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

fn inner(s: &str, include_slash: bool) -> usize {
    let mut state_grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    s.lines().for_each(|line| {
        let (x1, x2, y1, y2) = parse(line);
        if x1 == x2 || y1 == y2 {
            // straight line
            for y in smart_range_inclusive(y1, y2) {
                for x in smart_range_inclusive(x1, x2) {
                    state_grid[y][x] += 1;
                }
            }
        } else if include_slash {
            for tp in smart_range_inclusive(x1, x2)
                .zip(smart_range_inclusive(y1, y2)) {
                state_grid[tp.1][tp.0] += 1;
            }
        }
    });
    state_grid.iter().map(|vec|
        vec.iter().filter(|&&n| n > 1).count()
    ).sum()
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 5);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 12);
    }
}