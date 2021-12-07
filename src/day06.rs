use std::collections::HashMap;

fn total_cnt_recursion(start: usize, days: usize) -> usize {
    if days == 0 || start >= days {
        1
    } else {
        total_cnt_recursion(6, days - start - 1) + total_cnt_recursion(8, days - start - 1)
    }
}

fn total_cnt_memorized(
    start: usize,
    days: usize,
    mem: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if mem.contains_key(&(start, days)) {
        return mem.get(&(start, days)).unwrap().clone();
    }
    let cnt = if days == 0 || start >= days {
        1
    } else {
        total_cnt_memorized(6, days - start - 1, mem)
            + total_cnt_memorized(8, days - start - 1, mem)
    };
    mem.insert((start, days), cnt);
    cnt
}

fn main() {
    let numbers: Vec<usize> = "2,5,5,3,2,2,5,1,4,5,2,1,5,5,1,2,3,3,4,1,4,1,4,4,2,1,5,5,3,5,4,3,4,1,5,4,1,5,5,5,4,3,1,2,1,5,1,4,4,1,4,1,3,1,1,1,3,1,1,2,1,3,1,1,1,2,3,5,5,3,2,3,3,2,2,1,3,1,3,1,5,5,1,2,3,2,1,1,2,1,2,1,2,2,1,3,5,4,3,3,2,2,3,1,4,2,2,1,3,4,5,4,2,5,4,1,2,1,3,5,3,3,5,4,1,1,5,2,4,4,1,2,2,5,5,3,1,2,4,3,3,1,4,2,5,1,5,1,2,1,1,1,1,3,5,5,1,5,5,1,2,2,1,2,1,2,1,2,1,4,5,1,2,4,3,3,3,1,5,3,2,2,1,4,2,4,2,3,2,5,1,5,1,1,1,3,1,1,3,5,4,2,5,3,2,2,1,4,5,1,3,2,5,1,2,1,4,1,5,5,1,2,2,1,2,4,5,3,3,1,4,4,3,1,4,2,4,4,3,4,1,4,5,3,1,4,2,2,3,4,4,4,1,4,3,1,3,4,5,1,5,4,4,4,5,5,5,2,1,3,4,3,2,5,3,1,3,2,2,3,1,4,5,3,5,5,3,2,3,1,2,5,2,1,3,1,1,1,5,1"
        .split(',').map(|s| s.parse().unwrap()).collect();

    let sum1: usize = numbers.iter().map(|&v| total_cnt_recursion(v, 80)).sum();
    println!("q1={}", sum1);
    let sum2: usize = numbers
        .iter()
        .map(|&v| total_cnt_memorized(v, 256, &mut HashMap::new()))
        .sum();
    println!("q2={}", sum2);
}
