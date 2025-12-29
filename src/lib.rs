//! # Advent of Code solutions in Rust, tuned for speed.
//!
//! [![badge]][link]
//!
//! [badge]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey
//! [link]: https://github.com/maneatingape/advent-of-code-rust

// Portable SIMD API is enabled by "simd" feature.
#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]
// Configure rustdoc.
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, bitset, grid, hash, heap, integer, iter, math, md5, parse, point, slice, thread
);

library!(year2019 "Rescue Santa from deep space with a solar system voyage."
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25, intcode
);