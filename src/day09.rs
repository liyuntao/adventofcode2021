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

fn q1(input: &str) -> usize {
    let grid = parse_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut res: usize = 0;
    for y in 0..rows {
        for x in 0..cols {
            let val = grid[y][x];
            let cnt = all_valid_adjacent(x, y, cols, rows)
                .iter()
                .filter(|&(a, b)| grid[*b][*a] <= val)
                .count();
            if cnt == 0 {
                res = val as usize + 1 + res;
            }
        }
    }
    println!("res {}", res);
    res
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT))
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
}
