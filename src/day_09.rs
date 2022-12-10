use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

use Dir::*;
use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    NoToken(String),
    ParseInt(ParseIntError),
    UnrecognisedDir(String),
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    pub x: i32,
    pub y: i32
}

impl Pos {
    fn adjacent_to(self: &Pos, pos2: &Pos) -> bool {
        self.x.abs_diff(pos2.x) <= 1 &&
            self.y.abs_diff(pos2.y) <= 1
    }
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(UnrecognisedDir(s.to_string()))
        }
    }
}

fn move_head(pos: &Pos, dir: &Dir) -> Pos {
    match dir {
        Left => Pos { x: pos.x - 1, y: pos.y },
        Right => Pos { x: pos.x + 1, y: pos.y },
        Up => Pos { x: pos.x, y: pos.y + 1 },
        Down => Pos { x: pos.x, y: pos.y - 1 }
    }
}

fn move_tail(head_pos: &Pos, tail_pos: &Pos) -> Pos {
    if head_pos.adjacent_to(tail_pos) {
        tail_pos.clone()
    } else {
        Pos {
            x: tail_pos.x + (head_pos.x - tail_pos.x).signum(),
            y: tail_pos.y + (head_pos.y - tail_pos.y).signum()
        }
    }
}

fn tail_locs() -> Result<usize, Error> {
    let file = File::open("input-09.txt").map_err(|e| IO(e))?;
    let mut head_pos = Pos { x: 0, y: 0 };
    let mut tail_pos = Pos { x: 0, y: 0 };
    let mut tail_locs: HashSet<Pos> = HashSet::from([tail_pos]);
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        let mut parts = line.split_whitespace();
        let dir = parts.next()
            .ok_or(NoToken(line.clone()))?
            .parse::<Dir>()?;
        let steps = parts.next()
            .ok_or(NoToken(line.clone()))?
            .parse::<u32>().map_err(|e| ParseInt(e))?;
        for _ in 0..steps {
            head_pos = move_head(&head_pos, &dir);
            tail_pos = move_tail(&head_pos, &tail_pos);
            tail_locs.insert(tail_pos);
        }
    }
    Ok(tail_locs.len())
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_tail_locs() {
        println!("{}", tail_locs().unwrap());
    }
}