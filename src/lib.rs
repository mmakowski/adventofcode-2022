use std::fs::File;
use std::io::{self, BufRead};

fn aoc01a() -> io::Result<u64> {
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
    return Ok(max);
}

fn aoc01b() -> io::Result<u64> {
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
    return Ok(top3_sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc01a_output() {
        println!("{}", aoc01a().unwrap());
    }

    #[test]
    fn aoc01b_output() {
        println!("{}", aoc01b().unwrap());
    }

}
