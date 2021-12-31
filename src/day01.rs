use chrono::format::parse;

const INPUT: &'static str = include_str!("../input/day01.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn q1(input: &str) -> usize { // q1: count total increases window by 1
    let num_list: Vec<i32> = parse_input(input);
    count_total_incr(&num_list)
}

fn q2(input: &str) -> usize { // q2: count total increases window by 3
    let num_list: Vec<i32> = parse_input(input);
    let win_by_3then_sum: Vec<i32> = num_list.windows(3).map(|win| win.iter().sum()).collect();
    count_total_incr(&win_by_3then_sum)
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

fn count_total_incr(inputs: &[i32]) -> usize {
    let len = inputs.len();
    // inputs.array_windows::<2>()
    //     .filter(|&arr_win| arr_win[1] > arr_win[0])
    //     .count()
    inputs[0..len - 1].iter().zip(&inputs[1..len])
        .filter(|tuple| tuple.1 > tuple.0)
        .count()
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 7);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 5);
    }
}