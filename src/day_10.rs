use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

use Error::*;
use Op::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    EmptyLine,
    MissingArg,
    UnrecognisedOp(String),
    ParseInt(ParseIntError),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    AddX(i64),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next().ok_or(EmptyLine)? {
            "noop" => Ok(Noop),
            "addx" => parts.next()
                .ok_or(MissingArg)
                .and_then(|v| v.parse::<i64>().map_err(|e| ParseInt(e)))
                .map(|i| AddX(i)),
            other => Err(UnrecognisedOp(other.to_string()))
        }
    }
}

fn cycle_x_prod() -> Result<i64, Error> {
    let mut x: i64 = 1;
    let control_cycles = [20, 60, 100, 140, 180, 220];
    let mut prev_cycle: i64 = 1;
    let mut next_cycle: i64;
    let mut result: i64 = 0;
    let mut next_control = 0;
    let file = File::open("input-10.txt").map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let op = line.map_err(|e| IO(e))?.parse::<Op>()?;
        next_cycle = prev_cycle + match op {
            Noop => 1,
            AddX(_) => 2
        };
        if next_control < control_cycles.len() &&
            (prev_cycle == control_cycles[next_control] ||
                prev_cycle + 1 == control_cycles[next_control] && prev_cycle == next_cycle - 2) {
            result += x * control_cycles[next_control];
            next_control += 1
        }
        prev_cycle = next_cycle;
        if let AddX(i) = op {
            x += i;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_cycle_x_prod() {
        println!("{}", cycle_x_prod().unwrap());
    }
}