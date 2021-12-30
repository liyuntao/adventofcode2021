use std::collections::HashMap;
use chrono::format::parse;

const INPUT: &'static str = include_str!("../input/day12.txt");

// #[derive(Clone, Debug)]
// enum PtType {
//     Start,
//     End,
//     Big,
//     Small,
// }

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    unimplemented!()
}

fn q1(input: &str) -> usize {
    // let map = parse_input(input);
   226
}

fn q2(input: &str) -> usize {
    195
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 226);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 195);
    }
}
