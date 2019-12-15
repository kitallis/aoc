use std::fs::read_to_string;
use std::vec::Vec;
use std::option::Option::None;
use std::option::Option;
use std::option::Option::Some;

static PUZZLE_INPUT: &str = "2019/input/day2";

fn computer(mut codes: Vec<u64>) -> Vec<u64> {
    for position in (0..).step_by(4) {
        let operand_1 = codes[codes[position + 1 as usize] as usize];
        let operand_2 = codes[codes[position + 2 as usize] as usize];
        let op_result = codes[position + 3 as usize];

        codes[op_result as usize] =
            match codes[position] {
                99 => break,
                1 => operand_1 + operand_2,
                2 => operand_1 * operand_2,

                _ => unreachable!()
            };
    }

    codes
}

fn intcoder(codes: Vec<u64>, goal: u64) -> Option<u64> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions = codes.clone(); // reset memory

            instructions[1] = noun;
            instructions[2] = verb;

            if computer(instructions)[0] == goal { return Some(100 * noun + verb); }
        }
    }

    None
}

fn main() {
    let codes: Vec<u64> =
        read_to_string(PUZZLE_INPUT)
            .unwrap()
            .trim()
            .split(',')
            .map(|code| code.parse().unwrap())
            .collect();

    let mut instructions = codes.clone();
    instructions[1] = 12;
    instructions[2] = 2;

    println!("day2.a: {}, day2.b: {:?}",
             computer(instructions)[0],
             intcoder(codes.clone(), 19_690_720).unwrap())
}

#[cfg(test)]
mod day_2_tests {
    #[test]
    fn test_1() {
        assert_eq!(vec![2, 0, 0, 0, 99], super::computer(vec![1, 0, 0, 0, 99]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2, 3, 0, 6, 99], super::computer(vec![2, 3, 0, 3, 99]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2, 4, 4, 5, 99, 9801],
                   super::computer(vec![2, 4, 4, 5, 99, 0]));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
                   super::computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }
}
