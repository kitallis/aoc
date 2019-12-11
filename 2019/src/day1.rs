use std::fs::read_to_string;

static PUZZLE_INPUT: &str = "2019/input/day1";

fn fuel(mass: u64) -> u64 {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn fuel_of_fuel(mass: u64) -> u64 {
    let cost_of_fuel: u64 = fuel(mass);

    if cost_of_fuel == 0 {
        return cost_of_fuel;
    }

    cost_of_fuel + fuel_of_fuel(cost_of_fuel)
}

fn main() {
    let total_fuel_requirement =
        read_to_string(PUZZLE_INPUT)
            .unwrap()
            .lines()
            .fold(0, |aggregate: u64, line|
                aggregate + fuel(line.parse().unwrap()),
            );

    let total_fuel_of_fuel_requirement =
        read_to_string(PUZZLE_INPUT)
            .unwrap()
            .lines()
            .fold(0, |aggregate: u64, line|
                aggregate + fuel_of_fuel(line.parse().unwrap()),
            );


    println!("day1.a: {}, day1.b: {}", total_fuel_requirement, total_fuel_of_fuel_requirement)
}

#[cfg(test)]
mod day_1_tests {
    #[test]
    fn test_1() {
        assert_eq!(2, super::fuel(12));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, super::fuel(14));
    }

    #[test]
    fn test_3() {
        assert_eq!(654, super::fuel(1969));
    }

    #[test]
    fn test_4() {
        assert_eq!(33583, super::fuel(100756));
    }

    #[test]
    fn test_5() {
        assert_eq!(966, super::fuel_of_fuel(1969));
    }

    #[test]
    fn test_6() {
        assert_eq!(50346, super::fuel_of_fuel(100756));
    }
}
