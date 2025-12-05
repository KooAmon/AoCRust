use std::{fmt::Display, time::{Duration, Instant}};
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
    const YEAR_ROOT: &str = env!("CARGO_MANIFEST_DIR");
    let file_path = format!(r"{}\src\inputs\day{:02}", YEAR_ROOT, day);

    let result = std::fs::read_to_string(&file_path);

    match result {
        Ok(content) => return content,
        Err(_) => panic!("Failed to read input file '{}' for day:{}", file_path, day),
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
            let started = Instant::now();
            let part1 = self.part1();
            let elapsed = started.elapsed();
            println!("Day{:02} Part 1: {} ({})", part1.0, part1.1, PrettyDuration(elapsed));
        }

        if self.should_run_part2() {
            let started = Instant::now();
            let part2 = self.part2();
            let elapsed = started.elapsed();
            println!("Day{:02} Part 2: {} ({})", part2.0, part2.1, PrettyDuration(elapsed));
        }
    }
}

/// Local wrapper for pretty-printing durations.
pub struct PrettyDuration(pub Duration);

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.0.as_nanos();
        let (value, unit) = if nanos >= 1_000_000_000  { (nanos as f64 / 1_000_000_000.0, "s") }
                                       else if nanos >= 1_000_000 { (nanos as f64 / 1_000_000.0, "ms") }
                                       else if nanos >= 1_000     { (nanos as f64 / 1_000.0, "µs") }
                                       else                       { (nanos as f64, "ns") };

        let mut s = format!("{:.2}", value);
        if s.contains('.') {
            while s.ends_with('0') { s.pop(); }
            if s.ends_with('.') { s.pop(); }
        }

        write!(f, "{}{}", s, unit)
    }
}