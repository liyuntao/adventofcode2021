use itertools::Itertools;

const INPUT: &'static str = include_str!("../input/day11.txt");

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
        (x_i32 - 1, y_i32 - 1),
        (x_i32 - 1, y_i32 + 1),
        (x_i32 + 1, y_i32 - 1),
        (x_i32 + 1, y_i32 + 1),
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

fn is_all_zero(grid: &Vec<Vec<u8>>) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] > 0 {
                return false;
            }
        }
    }
    true
}

fn tick_one(grid: &mut Vec<Vec<u8>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut boom_total = 0;

    grid.iter_mut()
        .for_each(|inner| inner.iter_mut().for_each(|x| *x += 1));
    loop {
        let mut all_clean = true;
        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] > 9 {
                    all_clean = false;
                }
            }
        }
        if all_clean {
            break;
        }

        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] > 9 {
                    grid[y][x] = 0;
                    all_valid_adjacent(x, y, cols, rows)
                        .iter()
                        .for_each(|&(a, b)| {
                            if grid[b][a] != 0 {
                                grid[b][a] += 1;
                            }
                        });
                    boom_total += 1;
                }
            }
        }
    }
    boom_total
}

fn q1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    (0..100).map(|_i| tick_one(&mut grid)).sum()
}

fn q2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    (0..9999)
        .find_or_first(|i| {
            tick_one(&mut grid);
            is_all_zero(&grid)
        })
        .unwrap()
        + 1
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 1656);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 195);
    }
}
