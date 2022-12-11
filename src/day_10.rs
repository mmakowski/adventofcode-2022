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

fn x_vals() -> Result<Vec<i64>, Error> {
    let mut result: Vec<i64> = vec![];
    let mut x: i64 = 1;
    let file = File::open("input-10.txt").map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let op = line.map_err(|e| IO(e))?.parse::<Op>()?;
        let cycles =  match op {
            Noop => 1,
            AddX(_) => 2
        };
        for _ in 0..cycles {
            result.push(x)
        }
        if let AddX(i) = op {
            x += i;
        }
    }
    Ok(result)
}

fn cycle_x_prod() -> Result<i64, Error> {
    let control_cycles = [20, 60, 100, 140, 180, 220];
    let xs = x_vals()?;
    let mut result: i64 = 0;
    for cycle in control_cycles {
        result += (cycle as i64) * xs[cycle-1];
    }
    Ok(result)
}

fn drawing() -> Result<String, Error> {
    let mut result: String = String::new();
    let xs = x_vals()?;
    for i in 0..240 {
        if i % 40 == 0 {
            result.push('\n');
        }
        if (i as i64 % 40).abs_diff(xs[i]) <= 1 {
            result.push('#')
        } else {
            result.push(' ')
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

    #[test]
    fn print_drawing() {
        println!("{}", drawing().unwrap());
    }
}