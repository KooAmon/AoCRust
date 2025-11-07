use crate::utils::Args;
use clap::Parser;

// Macro to automatically generate module declarations and runners
macro_rules! aoc_days {
    ($($day:ident),*) => {
        // Module declarations
        $(mod $day;)*

        mod utils;

        #[tokio::main]
        async fn main() {
            let args = Args::parse();

            // Map day names to numbers and runner functions
            let day_runners: Vec<(u8, &str, fn(bool, bool))> = vec![
                $(
                    (
                        stringify!($day)
                            .strip_prefix("day")
                            .and_then(|s| s.parse::<u8>().ok())
                            .unwrap(),
                        stringify!($day),
                        $day::run,
                    ),
                )*
            ];

            // Filter days based on CLI argument
            let days_to_run: Vec<_> = if let Some(day_num) = args.day {
                day_runners
                    .into_iter()
                    .filter(|(num, _, _)| *num == day_num)
                    .collect()
            } else {
                day_runners
            };

            if days_to_run.is_empty() {
                eprintln!("Error: Day {} not found or not implemented", args.day.unwrap());
                return;
            }

            if args.concurrent {
                // Run each day concurrently
                let (part1, part2) = args.parts_to_run();
                let tasks: Vec<_> = days_to_run
                    .into_iter()
                    .map(|(_, name, run_fn)| {
                        tokio::spawn(async move {
                            println!("Running {}", name);
                            run_fn(part1, part2);
                        })
                    })
                    .collect();

                // Wait for all tasks to complete
                for task in tasks {
                    let _ = task.await;
                }
            } else {
                // Run days sequentially
                let (part1, part2) = args.parts_to_run();
                for (_, name, run_fn) in days_to_run {
                    println!("Running {}", name);
                    run_fn(part1, part2);
                }
            }
        }
    };
}

// Just list the days here
aoc_days!();
