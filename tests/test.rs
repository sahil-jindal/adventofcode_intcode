// Templates

// pub fn parse(_input: &str) -> Vec<u32> {
//     vec![]
// }
//
// pub fn part1(_input: &[u32]) -> u32 {
//     123
// }
//
// pub fn part2(_input: &[u32]) -> u32 {
//     456
// }

// use aoc::year2025::day00::*;
//
// const EXAMPLE: &str = "";
//
// #[test]
// fn part1_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part1(&input), 123);
// }
//
// #[test]
// fn part2_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part2(&input), 456);
// }

macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

test!(year2019
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);