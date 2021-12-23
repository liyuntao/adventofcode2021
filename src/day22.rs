extern crate regex;

use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input/day22.txt");

fn parse_line(line: &str) -> (bool, i32, i32, i32, i32, i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    }
    println!("{}", line);
    let is_on: bool = line.starts_with("on");
    let caps = RE.captures(line).unwrap();
    let arr: Vec<i32> = (1..=6).map(|i| caps.get(i).unwrap().as_str().parse::<i32>().unwrap())
        .collect();
    (is_on, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5])
}

fn parse_input(input: &str) -> Vec<(bool, i32, i32, i32, i32, i32, i32)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn q1(input: &str) -> usize {
    let mut xyz: HashSet<(i32, i32, i32)> = HashSet::new();

    parse_input(input).iter()
        .for_each(|&tp| {
            let is_on = tp.0;
            let z1 = max(-50, tp.5);
            let z2 = min(50, tp.6);
            let y1 = max(-50, tp.3);
            let y2 = min(50, tp.4);
            let x1 = max(-50, tp.1);
            let x2 = min(50, tp.2);
            iproduct!(z1..=z2, y1..=y2, x1..=x2)
                .for_each(|(x, y, z)| {
                    if is_on {
                        xyz.insert((x, y, z));
                    } else {
                        xyz.remove(&(x, y, z));
                    }
                });
        });
    xyz.len()
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

    const INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 590784);
    }

    // #[test]
    // fn q2_test() {
    //     assert_eq!(q2(INPUT), 195);
    // }
}
