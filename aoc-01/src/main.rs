use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut max: u64 = 0;
    let mut curr: u64 = 0;
    let file = File::open("input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                println!("{}", curr);
                if curr > max {
                    max = curr;
                }
                curr = 0;
            } else {
                curr += l.parse::<u64>().unwrap();
            }
        }
    }
    if curr > max {
        max = curr;
    }
    println!("max: {}", max);
    return Ok(());
}
