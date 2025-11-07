use clap::Parser;

/// Advent of Code 2015 Solutions
#[derive(Parser, Debug)]
#[command(name = "rustaoc")]
#[command(about = "Run Advent of Code solutions", long_about = None)]
pub struct Args {
    /// Specific day to run (1-25). If not provided, runs all days
    #[arg(short, long)]
    pub day: Option<u8>,

    /// Run only part 1 (if neither --part1 nor --part2 is specified, both run)
    #[arg(long, conflicts_with = "part2")]
    pub part1: bool,

    /// Run only part 2 (if neither --part1 nor --part2 is specified, both run)
    #[arg(long, conflicts_with = "part1")]
    pub part2: bool,

    /// Run days concurrently
    #[arg(short, long, default_value_t = false)]
    pub concurrent: bool,
}

impl Args {
    /// Returns (should_run_part1, should_run_part2)
    pub fn parts_to_run(&self) -> (bool, bool) {
        match (self.part1, self.part2) {
            (true, false) => (true, false),   // --part1 only
            (false, true) => (false, true),   // --part2 only
            _ => (true, true),                // default: both
        }
    }
}


pub fn load_input(day: u8) -> String {
    let filename = format!(r"E:\repos\Random Code\AdventOfCode\rustaocXXXyearXXX\src\inputs\day{:02}", day);

    let result = std::fs::read_to_string(filename);

    match result {
        Ok(content) => return content,
        Err(_) => panic!("Failed to read input with day:{}", day),
    }
}

pub trait AoCDay {
    fn new(part1: bool, part2: bool) -> Self;
    fn part1(&self) -> (u8, String);
    fn part2(&self) -> (u8, String);
    fn should_run_part1(&self) -> bool;
    fn should_run_part2(&self) -> bool;

    fn run(&self) {
        if self.should_run_part1() {
            let part1 = self.part1();
            println!("Day{:02} Part 1: {}", part1.0, part1.1);
        }

        if self.should_run_part2() {
            let part2 = self.part2();
            println!("Day{:02} Part 2: {}", part2.0, part2.1);
        }
    }
}