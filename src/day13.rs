use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input/day13.txt");

#[derive(Copy, Clone, Debug)]
enum FoldRule {
    X(usize),
    Y(usize),
}

fn parse_point(input: &str) -> HashSet<(usize, usize)> {
    input.lines()
        .filter(|line| line.contains(','))
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        }).collect()
}

fn parse_rule(input: &str) -> Vec<FoldRule> {
    input.lines()
        .filter(|line| line.starts_with("fold"))
        .map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            let n = parts[1].parse::<usize>().unwrap();
            if line.contains('x') {
                FoldRule::X(n)
            } else {
                FoldRule::Y(n)
            }
        }).collect()
}

fn mirror(pt: (usize, usize), rule: FoldRule) -> (usize, usize) {
    match rule {
        FoldRule::Y(n) => if pt.1 <= n { pt } else { (pt.0, 2 * n - pt.1) }
        FoldRule::X(n) => if pt.0 <= n { pt } else { (2 * n - pt.0, pt.1) }
    }
}

fn fold_all(points: HashSet<(usize, usize)>, rule: FoldRule) -> HashSet<(usize, usize)> {
    points.iter()
        .map(|&p| mirror(p, rule.clone()))
        .collect()
}

fn q1(input: &str) -> usize {
    let first_rule = parse_rule(input).first().unwrap().clone();
    fold_all(parse_point(input), first_rule).len()
}

fn q2(input: &str) {
    let mut matrix = vec![vec![false; 50]; 50];
    parse_rule(input).iter()
        .fold(parse_point(input), |points, &rule| fold_all(points, rule))
        .iter()
        .for_each(|&p| matrix[p.1][p.0] = true);
    matrix.iter().for_each(|row| {
        row.iter().for_each(|&b| print!("{}", if b { '#' } else { ' ' }));
        println!();
    });
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    q2(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 17);
    }
}
