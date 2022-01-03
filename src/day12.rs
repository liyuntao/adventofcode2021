use std::collections::HashMap;
use chrono::format::parse;

const INPUT: &'static str = include_str!("../input/day12.txt");

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut dict: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let str_vec: Vec<&str> = line.split('-').collect();
        let left = str_vec[0];
        let right = str_vec[1];
        if dict.contains_key(left) {
            dict.get_mut(left).unwrap().push(right);
        } else {
            dict.insert(left, vec![right]);
        }
        if dict.contains_key(right) {
            dict.get_mut(right).unwrap().push(left);
        } else {
            dict.insert(right, vec![left]);
        }
    });
    dict
}

fn is_uppercase(input: &str) -> bool {
    input.len() == 2 && input.chars().all(|c| c.is_uppercase())
}

fn is_lowercase(input: &str) -> bool {
    input.len() == 2 && input.chars().all(|c| c.is_lowercase())
}

fn dfs(path: Vec<&str>, dict: &HashMap<&str, Vec<&str>>) -> usize {
    let last_pt = *path.last().unwrap();
    if last_pt == "end" {
        1
    } else {
        let reachable_pts = dict.get(last_pt).unwrap();
        reachable_pts.iter()
            .filter(|&&pt| pt != "start")
            .filter(|&pt| is_uppercase(pt)
                || (is_lowercase(pt) && !path.contains(pt))
                || *pt == "end"
            )
            .map(|&pt| {
                let mut clone = path.clone();
                clone.push(pt);
                dfs(clone, dict)
            }).sum()
    }
}

fn dfs2(path: Vec<&str>, dict: &HashMap<&str, Vec<&str>>, must_twice: &str) -> usize {
    let specific_char_cnt = path.iter()
        .filter(|&&pt| pt == must_twice).count();
    if specific_char_cnt > 2 {
        return 0;
    }

    let last_pt = *path.last().unwrap();
    if last_pt == "end" {
        let specific_char_cnt = path.iter().filter(|&&pt| pt == must_twice).count();
        if specific_char_cnt == 2 {
            1
        } else {
            0
        }
    } else {
        let reachable_pts = dict.get(last_pt).unwrap();
        reachable_pts.iter()
            .filter(|&&pt| pt != "start")
            .filter(|&&pt| is_uppercase(pt)
                || must_twice == pt
                || (is_lowercase(pt) && !path.contains(&pt))
                || pt == "end"
            )
            .map(|&pt| {
                let mut clone = path.clone();
                clone.push(pt);
                dfs2(clone, dict, must_twice)
            }).sum()
    }
}

fn q1(input: &str) -> usize {
    let dict = parse_input(input);
    dfs(vec!["start"], &dict)
}

fn q2(input: &str) -> usize {
    let dict = parse_input(input);
    dict.keys()
        .filter(|&&key| is_lowercase(key))
        .map(|&low_key| dfs2(vec!["start"], &dict, low_key))
        .sum::<usize>() + dfs(vec!["start"], &dict)
}

pub(crate) fn run() {
    println!("q1={}", q1(INPUT));
    println!("q2={}", q2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn q1_test() {
        assert_eq!(q1(INPUT), 226);
    }

    #[test]
    fn q2_test() {
        assert_eq!(q2(INPUT), 3509);
    }
}
