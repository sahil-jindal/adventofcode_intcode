use aoc::util::ansi::*;
use aoc::util::parse::*;
use std::env::args;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

const YEAR: u32 = 2019;

struct Solution {
    day: u32,
    wrapper: fn(&str) -> (String, String),
}

fn main() {
    // Parse command line options
    let mut iter = args().flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let day = iter.next();


    // Run selected solutions.
    let (stars, duration) = year2019()
        .iter()
        .filter(|s| day.is_none_or(|d| d == s.day))
        .fold((0, Duration::ZERO), run_solution);

    // Optionally print totals.
    if args().any(|arg| arg == "--totals") {
        println!("{BOLD}{YELLOW}⭐ {stars}{RESET}");
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}

fn run_solution((stars, duration): (u32, Duration), solution: &Solution) -> (u32, Duration) {
    let Solution { day, wrapper } = solution;
    let path = format!("input/day{day:02}.txt");

    if let Ok(data) = read_to_string(&path) {
        let instant = Instant::now();
        let (part1, part2) = wrapper(&data);
        let elapsed = instant.elapsed();

        println!("{BOLD}{YELLOW}{YEAR} Day {day}{RESET}");
        println!("    Part 1: {part1}");
        println!("    Part 2: {part2}");

        (stars + 2, duration + elapsed)
    } else {
        eprintln!("{BOLD}{RED}{YEAR} Day {day}{RESET}");
        eprintln!("    Missing input!");
        eprintln!("    Place input file in {BOLD}{WHITE}{path}{RESET}");

        (stars, duration)
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    day: stringify!($day).unsigned(),
                    wrapper: |data: &str| {
                        use aoc::$year::$day::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();

                        (part1, part2)
                    }
                }
            ,)*]
        }
    }
}

run!(year2019
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);
