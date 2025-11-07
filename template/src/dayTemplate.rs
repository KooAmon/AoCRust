use crate::utils::{AoCDay, load_input};

pub struct DayXXX2dayXXX(String, bool, bool);

pub fn run(part1: bool, part2: bool) {
    let day = DayXXX2dayXXX::new(part1, part2);
    day.run();
}

impl AoCDay for DayXXX2dayXXX {
    fn new(part1: bool, part2: bool) -> Self {
        Self(load_input(XXX2dayXXX), part1, part2)
    }

    fn part1(&self) -> (u8, String) {
        return (XXXdayXXX, String::new());
    }

    fn part2(&self) -> (u8, String) {
        return (XXXdayXXX, String::new());
    }

    fn should_run_part1(&self) -> bool {
        self.1
    }

    fn should_run_part2(&self) -> bool {
        self.2
    }
}