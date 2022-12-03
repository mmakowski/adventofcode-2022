use std::fs::File;
use std::io::{self, BufRead};

fn max_calories() -> io::Result<u64> {
    let mut max: u64 = 0;
    let mut curr: u64 = 0;
    let file = File::open("input-01.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l.is_empty() {
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
    Ok(max)
}

fn top_three_calories() -> io::Result<u64> {
    let mut totals: Vec<u64> = Vec::new();
    let mut curr: u64 = 0;
    let file = File::open("input-01.txt")?;
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
    let top3_sum = totals[0] + totals[1] + totals[2];
    Ok(top3_sum)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_max_calories() {
        println!("{}", max_calories().unwrap());
    }

    #[test]
    fn print_top_three_calories() {
        println!("{}", top_three_calories().unwrap());
    }
}
