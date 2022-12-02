use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut totals: Vec<u64> = Vec::new();
    let mut curr: u64 = 0;
    let file = File::open("input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                totals.push(curr);
                curr = 0;
            } else {
                curr += l.parse::<u64>().unwrap();
            }
        }
    }
    totals.push(curr);
    totals.sort();
    totals.reverse();
    println!("max: {}", totals[0]);
    println!("top 3 total: {}", totals[0] + totals[1] + totals[2]);
    return Ok(());
}
