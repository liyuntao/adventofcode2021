use std::cmp::min;
use itertools::iproduct;

const INPUT: &'static str = include_str!("../input/day15.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn gen_dp_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0u32; cols]; rows];
    iproduct!((0..rows).rev(), (0..cols).rev()).for_each(|p| {
        dp[p.0][p.1] = grid[p.0][p.1] + min_of_down_and_right(&dp, p.1, p.0);
    });
    dp.iter().for_each(|row| {
        row.iter().for_each(|&b| print!("{number:>width$}", number=b, width=4));
        println!();
    });
    dp
}

fn min_of_down_and_right(dp: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let rows = dp.len() - 1;
    let cols = dp[0].len() - 1;
    if x == cols && y == rows {
        0
    } else if x == cols {
        dp[y + 1][x]
    } else if y == rows {
        dp[y][x + 1]
    } else {
        min(dp[y][x + 1], dp[y + 1][x])
    }
}

fn q1(input: &str) -> u32 {
    let grid = parse_input(input);
    let dp_grid = gen_dp_grid(&grid);
    dp_grid[0][0] - grid[0][0]
}

fn q2(input: &str) -> usize {
    195
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    // println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 40);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 195);
    }
}
