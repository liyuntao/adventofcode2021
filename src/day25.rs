use chrono::format::parse;

const INPUT: &'static str = include_str!("../input/day25.txt");

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn gen_mark_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    unimplemented!()
}

// fn is_all_false(mark_grid: &Vec<Vec<bool>>) -> bool {
//     mark_grid.iter().all()
// }

fn tick_flow(grid: &mut Vec<Vec<char>>, mark_grid: Vec<Vec<bool>>) {
    unimplemented!()
}

fn q1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut step_cnt = 0;
    1
}

fn q2(input: &str) -> usize {
    1
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 590784);
    }

    // #[test]
    // fn q2_test() {
    //     assert_eq!(q2(INPUT), 195);
    // }
}
