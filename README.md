# Advent of Code

## Language: Rust

When using this codebase, you can run individual days of the Advent of Code challenges implemented in Rust. Each day is organized into its own module, and you can easily add more days or create a new Year.

### How to Add a New Year

1. Open PowerShell to the root directory (same directory as this file)
2. Run Rust-Prep.ps1 with the desired year and day
3. This will create a new directory for the year and set up the necessary files.

```powershell
cd <path-to-root-directory>
.\Rust-Prep.ps1 -Year 2016 -Day 01
```
### How to Add a New Day to an Existing Year

1. In PowerShell run Rust-Prep.ps1 with the desired year and day
2. This will create a new module for the day and set up the necessary files.

> NOTE! This does not have to be done from the root directory, but it can be.

```powershell
# if in the root directory
.\Rust-Prep.ps1 -Year 2015 -Day 09
# if in the year directory
..\Rust-Prep.ps1 -Year 2015 -Day 09
```

### How to Run a Specific Year

To run a specific year, navigate to that year's directory and use Cargo to execute the program.

```powershell
cd <path-to-year-directory>
cargo run
# Example for 2015
cd e:\Repos\Random Code\AdventOfCode\rustaoc2015
cargo run
```

### How to Run a Specific Year concurrently

To run a specific year concurrently, you can use the `--concurrent` flag when executing the program.

```powershell
# This will run all days of the year concurrently.
cargo run -- --concurrent
```

### How to Run a Specific Day

To run a specific day, you can set the 'day' parameter when executing the program.

```powershell
# This will run Day 1 of the Advent of Code challenges.
cargo run -- --day 1
```

### How to Run a Specific Part

To run a specific part, you can set the 'part1' or 'part2' parameter when executing the program.

```powershell
# This will run all part 1s of the Advent of Code challenges.
cargo run -- --part1
```

### How to Run a Specific Day and Part

To run a specific day and part, you can set the day and the 'part1' or 'part2' parameter when executing the program.

```powershell
# This will run day 1 part 1 of the Advent of Code challenges.
cargo run -- --day 1 --part1
```

### How to Update a part of a day

The code is already setup to be modular. Each day has its own module under the year directory. You can modify the code for each day in its respective module file.

Since running the Rust-Prep.ps1 script creates boilerplate code and downloads the input for the day, you can focus on implementing the logic for each day's challenge.

The logic for each day should be implemented in the `part1` and `part2` methods of the respective day's struct.

Notice the example below for Day 1:

```rust
// Example: src/day01.rs
use crate::utils::{AoCDay, load_input};

pub struct Day01(String);

pub fn run() {
    let day = Day01::new();
    day.run();
}

impl AoCDay for Day01 {
    fn new() -> Self {
        Self(load_input(1))
    }

    fn part1(&self) -> (u8, usize) {
        return (1, self.0.len() - 2*(self.0.matches(')').count()));
    }

    fn part2(&self) -> (u8, usize) {
        let mut floor: i32 = 0;

        for (i, c) in self.0.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _   => panic!("Unexpected character"),
            };

            if floor < 0 { return (1, i + 1); }
        }

        panic!("Never entered the basement");
    }
}
```

Each part returns a tuple where the first element is the part number and the second element is the result.

This structure allows for the days to be ran asynchronously and independently but still be organized under a common interface.
