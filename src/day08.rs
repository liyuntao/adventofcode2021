extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input/day08.txt");



fn q1(s: &str) -> usize {
    s.lines()
        .map(|line| line.split('|').collect())
        .map(|arr: Vec<&str>| arr[1].trim())
        .map(|line2 | line2.split(' ').collect())
        .map(|arr2: Vec<&str>| arr2.iter()
            .filter(|word| word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7)
            .count())
        .sum()
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

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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
