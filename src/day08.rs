extern crate regex;

use crate::day08::LedPart::{Bottom, DownLeft, DownRight, Mid, UpLeft, UpRight, TOP};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input/day08.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum LedPart {
    TOP,
    UpLeft,
    UpRight,
    Mid,
    DownLeft,
    DownRight,
    Bottom,
}

fn part_to_num(mut parts: Vec<LedPart>) -> usize {
    parts.sort();
    if parts == vec![TOP, UpLeft, UpRight, DownLeft, DownRight, Bottom] {
        0
    } else if parts == vec![UpRight, DownRight] {
        1
    } else if parts == vec![TOP, UpRight, Mid, DownLeft, Bottom] {
        2
    } else if parts == vec![TOP, UpRight, Mid, DownRight, Bottom] {
        3
    } else if parts == vec![UpLeft, UpRight, Mid, DownRight] {
        4
    } else if parts == vec![TOP, UpLeft, Mid, DownRight, Bottom] {
        5
    } else if parts == vec![TOP, UpLeft, Mid, DownLeft, DownRight, Bottom] {
        6
    } else if parts == vec![TOP, UpRight, DownRight] {
        7
    } else if parts == vec![TOP, UpLeft, UpRight, Mid, DownLeft, DownRight, Bottom] {
        8
    } else {
        9
    }
}

/**
*   8
* 6   8
*   7
* 4   9
*   7
*/
fn gen_mapping(line: &str) -> HashMap<char, LedPart> {
    let parts: Vec<&str> = line.split(' ').collect();

    let mut cnt_map: HashMap<char, usize> = HashMap::new();
    for c in line.chars().filter(|&c| c != ' ') {
        *cnt_map.entry(c).or_insert(0) += 1;
    }
    let mut map = HashMap::new();
    for (c, cnt) in cnt_map {
        if cnt == 6 {
            map.insert(c, UpLeft);
        } else if cnt == 4 {
            map.insert(c, DownLeft);
        } else if cnt == 9 {
            map.insert(c, DownRight);
        } else if cnt == 8 {
            let len2word = parts.iter().find_or_first(|&s| s.len() == 2).unwrap();
            if len2word.contains(c) {
                map.insert(c, UpRight);
            } else {
                map.insert(c, TOP);
            }
        } else if cnt == 7 {
            let len4word = parts.iter().find_or_first(|&s| s.len() == 4).unwrap();
            if len4word.contains(c) {
                map.insert(c, Mid);
            } else {
                map.insert(c, Bottom);
            }
        }
    }
    map
}

fn map_to_n(part: &str, dict: &HashMap<char, LedPart>) -> usize {
    let parts = part
        .chars()
        .map(|c| dict.get(&c).unwrap())
        .map(|v| v.clone())
        .collect();
    part_to_num(parts)
}

fn q2_inner(raw_line: &str) -> usize {
    let part: Vec<&str> = raw_line.split('|').collect();
    let left_part = part[0].trim();
    let dict = gen_mapping(left_part);
    let right_parts: Vec<&str> = part[1].trim().split(' ').collect();
    right_parts
        .iter()
        .map(|&part| map_to_n(part, &dict))
        .fold(0, |acc, b| acc * 10 + b)
}

fn q1(s: &str) -> usize {
    s.lines()
        .map(|line| line.split('|').collect())
        .map(|arr: Vec<&str>| arr[1].trim())
        .map(|line2| line2.split(' ').collect())
        .map(|arr2: Vec<&str>| {
            arr2.iter()
                .filter(|word| {
                    word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7
                })
                .count()
        })
        .sum()
}

fn q2(s: &str) -> usize {
    s.lines().map(|line| q2_inner(line)).sum()
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 26);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 61229);
    }
}
