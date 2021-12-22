const INPUT: &'static str = include_str!("../input/day10.txt");

fn detect_corrupted_points(line: &str) -> usize {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '{' || c == '<' || c == '[' || c == '(' {
            stack.push(c);
        } else {
            let last_char = stack.pop().unwrap();
            if c as u8 != (last_char as u8 + 1) && c as u8 != (last_char as u8 + 2) {
                return char_point_q1(c);
            }
        }
    }
    return 0;
}

fn char_point_q1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn char_point_q2(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn count_points_q2(line: &str) -> usize {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '{' || c == '<' || c == '[' || c == '(' {
            stack.push(c);
        } else {
            let last_char = stack.pop().unwrap();
            if c as u8 != (last_char as u8 + 1) && c as u8 != (last_char as u8 + 2) {
                return 0;
            }
        }
    }
    // here q2
    stack.reverse();
    stack.iter()
        .fold(0, |n, &c| n * 5 + char_point_q2(c))
}

fn q1(input: &str) -> usize {
    input.lines().map(|line| detect_corrupted_points(line)).sum()
}

fn q2(input: &str) -> usize {
    let mut p_list: Vec<usize> = input.lines()
        .map(|line| count_points_q2(line))
        .filter(|&n| n > 0)
        .collect();
    p_list.sort();
    p_list[p_list.len() / 2]
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 26397);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 288957);
    }
}
