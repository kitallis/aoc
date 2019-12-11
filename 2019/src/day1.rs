use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static PUZZLE_INPUT: &str = "2019/input/day1";

fn main() {
    println!("Hello, world!");
    let file = File::open(PUZZLE_INPUT).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}. {}", index + 1, line);
    }
}
