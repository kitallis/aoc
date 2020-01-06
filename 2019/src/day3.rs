use std::fs::read_to_string;
use std::vec::Vec;

static PUZZLE_INPUT: &str = "2019/input/day3";

type Point = (i16, i16);

fn parse_cmd(cmd: &str) {
    match &cmd[..1] {
        "U" => println!("{}", cmd),
        "R" => println!("{}", cmd),
        "D" => println!("{}", cmd),
        "L" => println!("{}", cmd),

        _ => unreachable!()
    }
}

fn main() {
    let input =
        read_to_string(PUZZLE_INPUT)
            .unwrap()
            .trim()
            .to_string()
            .clone();

    let wire_paths: Vec<Vec<&str>> =
        input.split('\n')
            .map(|wire| wire.trim().split(',').collect())
            .collect();

    for wire_path in wire_paths[0] {
        parse_cmd(&wire_path)
    }

    println!("{:?}", wire_paths)
}

#[cfg(test)]
mod day_3_tests {
    #[test]
    fn test_1() {}
}
