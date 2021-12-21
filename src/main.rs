#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use itertools::iproduct;

mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    day09::run();

    // iproduct!(0..5, 0..7)
    //     .for_each(|t| println!("{:?}", t))
}