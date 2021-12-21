use itertools::{iproduct, Itertools};

const INPUT: &'static str = include_str!("../input/day09.txt");

fn parse_grid(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn all_valid_adjacent(x: usize, y: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let x_i32 = x as i32;
    let y_i32 = y as i32;
    vec![
        (x_i32 - 1, y_i32),
        (x_i32 + 1, y_i32),
        (x_i32, y_i32 - 1),
        (x_i32, y_i32 + 1),
    ]
    .iter()
    .filter(|&(a, b)| *a >= 0 && *b >= 0 && *a < (m as i32) && *b < (n as i32))
    .map(|&(a, b)| (a as usize, b as usize))
    .collect()
}

fn all_low_pts(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    iproduct!(0..cols, 0..rows)
        .filter(|&(x, y)| {
            let val = grid[y][x];
            let cnt = all_valid_adjacent(x, y, cols, rows)
                .iter()
                .filter(|&(a, b)| grid[*b][*a] <= val)
                .count();
            cnt == 0
        })
        .map(|(x, y)| (x, y, grid[y][x] as usize))
        .collect()
}

fn q1(input: &str) -> usize {
    let grid = parse_grid(input);
    all_low_pts(&grid).iter().map(|&(x, y, val)| val + 1).sum()
}

fn q2(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut basin_list: Vec<usize> = all_low_pts(&grid)
        .iter()
        .map(|&(x, y, _val)| basin_size(&grid, x, y))
        .collect();
    basin_list.sort();
    println!("{:?}", basin_list);
    basin_list[basin_list.len() - 1]
        * basin_list[basin_list.len() - 2]
        * basin_list[basin_list.len() - 3]
}

fn basin_size(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    dfs(grid, x, y, &mut seen);
    let rows = seen.len();
    let cols = seen[0].len();
    iproduct!(0..cols, 0..rows)
        .filter(|&(x, y)| seen[y][x])
        .count()
}

fn dfs(grid: &Vec<Vec<u8>>, x: usize, y: usize, seen: &mut Vec<Vec<bool>>) {
    let val = grid[y][x];
    if val >= 9 {
        return;
    }
    seen[y][x] = true;
    let rows = grid.len();
    let cols = grid[0].len();
    all_valid_adjacent(x, y, cols, rows)
        .iter()
        .for_each(|&(a, b)| {
            if grid[b][a] > val && seen[b][a] == false {
                dfs(grid, a, b, seen)
            }
        });
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
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
    fn q1_test() {
        assert_eq!(q1(INPUT), 15);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 1134);
    }
}
