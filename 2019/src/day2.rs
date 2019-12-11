use std::fs::read_to_string;
use std::vec::Vec;

static PUZZLE_INPUT: &str = "2019/input/day2";

fn computer(mut codes: Vec<u64>) -> Vec<u64> {
    for position in (0..).step_by(4) {
        if codes[position] == 99 { break; }

        let operand_1 = codes[codes[position + 1 as usize] as usize];
        let operand_2 = codes[codes[position + 2 as usize] as usize];
        let op_result = codes[position + 3 as usize];

        codes[op_result as usize] =
            match codes[position] {
                1 => operand_1 + operand_2,
                2 => operand_1 * operand_2,
                _ => unreachable!()
            };
    }

    codes
}

fn main() {
    let mut codes: Vec<u64> =
        read_to_string(PUZZLE_INPUT)
            .unwrap()
            .trim()
            .split(',')
            .map(|code| code.parse().unwrap())
            .collect();

    codes[1] = 12;
    codes[2] = 2;

    println!("{}", computer(codes)[0])
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
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], super::computer(vec![2, 4, 4, 5, 99, 0]));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], super::computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }
}
