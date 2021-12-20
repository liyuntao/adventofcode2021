// const INPUT: &'static str = include_str!("../input/day03.txt");
//
// fn get_last_n(inputs: Vec<usize>, total: usize, preserve_major: bool) -> usize {
//     if inputs.len() == 1 {
//         inputs[0]
//     } else {
//         let index = total - 1;
//         let cur_len = inputs.len();
//         let bit1_count = inputs.iter().filter(|&n| (n >> index) & 1 == 1).count();
//         let major_bit = (2 * bit1_count >= cur_len) as usize;
//         let preserve_bit = if preserve_major { major_bit } else { !(major_bit != 0) as usize };
//         let new_inputs = inputs.into_iter()
//             .filter(|&n| (n >> index) & 1 == preserve_bit)
//             .collect();
//         get_last_n(new_inputs, index, preserve_major)
//     }
// }
//
// fn q1(s: &str) -> usize {
//     let inputs: Vec<&str> = s.lines().collect();
//     const N: usize = 12;
//     let half_row_count = inputs.len() / 2;
//     let reducer = |mut sum_arr: [usize; N], n: &usize| -> [usize; N] {
//         (0..N).for_each(|index|
//             sum_arr[index] += (N >> (N - 1 - index)) & 1);
//         sum_arr
//     };
//
//     let input_num: Vec<usize> = inputs.iter()
//         .map(|str| usize::from_str_radix(&str, 2).unwrap())
//         .collect();
//     let gamma_rate = input_num.iter()
//         .fold([0usize; N], reducer)
//         .map(|cnt| (cnt > half_row_count) as usize)
//         .iter().enumerate()
//         .fold(0usize, |mut acc, (i, n)| {
//             acc += n << (n - 1 - i);
//             acc
//         });
//     gamma_rate * (((1 << N) - 1) ^ gamma_rate)
// }
//
// fn q2(s: &str) -> usize {
//     let inputs: Vec<&str> = s.lines().collect();
//     let n: usize = inputs[0].len();
//     get_last_n(input_num.clone(), n, true)
//         * get_last_n(input_num, n, false)
// }
//
// pub(crate) fn run() {
//     println!("q1={}", q1(INPUT));
//     println!("q2={}", q2(INPUT));
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     const INPUT: &str = "00100
// 11110
// 10110
// 10111
// 10101
// 01111
// 00111
// 11100
// 10000
// 11001
// 00010
// 01010";
//
//     #[test]
//     fn q1_test() {
//         assert_eq!(q1(INPUT), 198);
//     }
//
//     #[test]
//     fn q2_test() {
//         assert_eq!(q2(INPUT), 230);
//     }
// }