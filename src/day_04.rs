use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn parse_range(s: &str) -> Result<Range<u64>, &str> {
    let mut parts = s.split("-");
    let mut next_num = || { parts.next().unwrap().parse::<u64>().unwrap() };
    let start = next_num();
    let end = next_num();
    assert!(parts.next().is_none());
    Ok(start..end+1)
}

fn is_subrange(sub: &Range<u64>, sup: &Range<u64>) -> bool {
    sup.contains(&sub.start) && sup.contains(&(sub.end - 1))
}

fn fully_contains() -> io::Result<u64> {
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

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_fully_contains() {
        println!("{}", fully_contains().unwrap());
    }
}
