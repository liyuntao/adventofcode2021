use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

enum MoveIns {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_to_ins(line: &str) -> MoveIns {
    let parts: Vec<&str> = line.split(' ').collect();
    let val: i32 = parts[1].parse().unwrap();
    match parts[0] {
        "forward" => MoveIns::Forward(val),
        "up" => MoveIns::Up(val),
        _ => MoveIns::Down(val),
    }
}

fn main() {
    let inputs: Vec<String> = read_input("day02.txt").unwrap();

    let q1_interpreter = |mut coordinate: (i32, i32), ins: MoveIns| -> (i32, i32) {
        match ins {
            MoveIns::Forward(x) => coordinate.0 += x,
            MoveIns::Up(x) => coordinate.1 -= x,
            MoveIns::Down(x) => coordinate.1 += x,
        };
        coordinate
    };

    let q2_interpreter = |mut silly_state: (i32, i32, i32), ins: MoveIns| -> (i32, i32, i32) {
        match ins {
            MoveIns::Down(x) => silly_state.2 += x,
            MoveIns::Up(x) => silly_state.2 -= x,
            MoveIns::Forward(x) => {
                silly_state.0 += x;
                silly_state.1 += x * silly_state.2
            },
        };
        silly_state
    };

    let tpl = inputs.iter().map(|line| parse_to_ins(line))
        .fold((0, 0), q1_interpreter);
    println!("{:?}", tpl);
    println!("q1={}", tpl.0 * tpl.1);

    let tpl2 = inputs.iter().map(|line| parse_to_ins(line))
        .fold((0, 0, 0), q2_interpreter);
    println!("q2={}", tpl2.0 * tpl2.1);
}
