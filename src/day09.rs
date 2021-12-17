use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input/day09.txt");

fn parse1(s: &str) {
    let grid: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", grid)
}

pub(crate) fn run() {
    // parse1(INPUT)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 15);
    }
}