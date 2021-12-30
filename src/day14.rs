use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input/day14.txt");

fn parse_input(input: &str) -> String {
    input.lines().collect::<Vec<&str>>()[0].to_string()
}

fn parse_dict(input: &str) -> HashMap<(char, char), char> {
    input
        .lines()
        .filter(|line| line.contains('-'))
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            ((chars[0], chars[1]), chars[6])
        })
        .collect()
}

fn tick(chars: Vec<char>, dict: &HashMap<(char, char), char>) -> Vec<char> {
    let last_char = *chars.last().unwrap();
    // array concat
    let mut concat_arr = chars
        .windows(2)
        .map(|arr| [arr[0], *dict.get(&(arr[0], arr[1])).unwrap()])
        .fold(vec![], |mut acc, inc| {
            acc.extend_from_slice(&inc);
            acc
        });
    concat_arr.push(last_char);
    concat_arr
}

fn q1(input: &str) -> usize {
    let line = parse_input(input);
    let dict = parse_dict(input);
    let input_chars: Vec<char> = line.chars().collect();

    let result = (0..10).fold(input_chars, |acc, _i| tick(acc, &dict));
    let cnt_map = result.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let max_cnt = cnt_map.iter().max_by(|&a, &b| a.1.cmp(b.1)).unwrap().1;
    let min_cnt = cnt_map.iter().min_by(|&a, &b| a.1.cmp(b.1)).unwrap().1;
    max_cnt - min_cnt
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 1588);
    }
}
