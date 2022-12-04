use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::ops::Range;

use crate::day_04::Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    NotEnoughLineParts,
    NotEnoughRangeParts,
    ParseError(ParseIntError),
}

fn parse_range(s: &str) -> Result<Range<u64>, Error> {
    let mut parts = s.split("-");
    let mut next_num = || {
        parts.next().ok_or(NotEnoughRangeParts)
            .and_then(|s| s.parse::<u64>().map_err(|e| ParseError(e)))
    };
    let start = next_num()?;
    let end = next_num()?;
    assert!(parts.next().is_none());
    Ok(start..end + 1)
}

fn is_subrange(sub: &Range<u64>, sup: &Range<u64>) -> bool {
    sup.contains(&sub.start) && sup.contains(&(sub.end - 1))
}

fn fully_contains_count() -> io::Result<u64> {
    let mut count: u64 = 0;
    let file = File::open("input-04.txt")?;
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut parts = line.split(",");
        let mut next_range = || { parse_range(parts.next().unwrap()).unwrap() };
        let r1 = next_range();
        let r2 = next_range();
        assert!(parts.next().is_none());
        if is_subrange(&r1, &r2) || is_subrange(&r2, &r1) {
            count += 1;
        }
    }
    Ok(count)
}

fn overlaps(r1: &Range<u64>, r2: &Range<u64>) -> bool {
    r1.contains(&r2.start) ||
        r1.contains(&(r2.end - 1)) ||
        is_subrange(r1, r2)
}

fn overlaps_count() -> Result<u64, Error> {
    let mut count: u64 = 0;
    let file = File::open("input-04.txt").map_err(|e| IO(e))?;
    for line_or_error in io::BufReader::new(file).lines() {
        let line = line_or_error.map_err(|e| IO(e))?;
        let mut parts = line.split(",");
        let mut next_range = || {
            parts.next().ok_or(NotEnoughLineParts)
                .and_then(|s| parse_range(s))
        };
        let r1 = next_range()?;
        let r2 = next_range()?;
        assert!(parts.next().is_none());
        if overlaps(&r1, &r2) {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_fully_contains_count() {
        println!("{}", fully_contains_count().unwrap());
    }

    #[test]
    fn print_overlaps_count() {
        println!("{}", overlaps_count().unwrap());
    }
}
