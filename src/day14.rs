use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input/day13.txt");


fn parse_input(input: &str) -> String {
    input.lines().collect::<Vec<&str>>()[0].to_string()
}

fn parse_map(input: &str) -> HashMap<(char, char), char> {
    input.lines()
        .filter(|line| line.contains('-'))
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            ((chars[0], chars[1]), chars[6])
        }).collect()
}

fn

fn q1(input: &str) -> usize {
    let line = parse_input(input);
    let input_chars: Vec<char> = line.chars().collect();
    // input_chars.windows(2)
    println!("{}", line);
    1588
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
