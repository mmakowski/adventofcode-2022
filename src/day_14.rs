use std::io;
use std::num::ParseIntError;
use std::ops::Range;
use crate::day_14::map::AddResult;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    EmptyCoords(String),
    EmptyPoints,
    ParseInt(ParseIntError),
    CoordsOutOfBounds(usize, usize, Range<usize>, Range<usize>)
}

fn count_resting() -> Result<u32, Error> {
    let mut map = parse::parse("input-14.txt")?;
    let mut result = 0;
    loop {
        match map.add_sand(500, 0)? {
            AddResult::Rest => result += 1,
            AddResult::Fall => break
        }
    }
    println!("{map:?}");
    Ok(result)
}

mod map {
    use std::fmt::{Debug, Formatter, Write};
    use std::ops::Range;

    use AddResult::*;
    use FieldContent::*;

    use super::Error;
    use super::Error::CoordsOutOfBounds;

    #[derive(Clone, Copy, Eq, PartialEq)]
    pub enum FieldContent {
        Empty,
        Rock,
        RestingSand
    }

    pub enum AddResult {
        Rest,
        Fall
    }

    pub struct Map {
        fields: Vec<FieldContent>,
        x_range: Range<usize>,
        y_range: Range<usize>
    }

    impl Map {
        pub fn new(x_range: Range<usize>, y_range: Range<usize>) -> Map {
            let width = x_range.len();
            let height = y_range.len();
            Map { fields: vec![Empty; width * height], x_range, y_range }
        }

        pub fn set_field(&mut self, x: usize, y: usize, f: FieldContent) -> Result<(), Error> {
            let idx = self.to_vec_idx(x, y)?;
            self.fields[idx] = f;
            Ok(())
        }

        pub fn add_sand(&mut self, start_x: usize, start_y: usize) -> Result<AddResult, Error> {
            let mut x = start_x;
            let mut y = start_y;
            loop {
                if !(self.x_range.contains(&x) && self.y_range.contains(&y)) {
                    return Ok(Fall)
                }
                if !(self.y_range.contains(&(y + 1))) {
                    return Ok(Fall)
                }
                if self.field(x, y + 1)? == Empty {
                    y += 1;
                } else if !(self.x_range.contains(&(x - 1))) {
                    return Ok(Fall)
                } else if self.field(x - 1, y + 1)? == Empty {
                    x -= 1;
                    y += 1;
                } else if !(self.x_range.contains(&(x + 1))) {
                    return Ok(Fall)
                } else if self.field(x + 1, y + 1)? == Empty {
                    x += 1;
                    y += 1;
                } else {
                    self.set_field(x, y, RestingSand)?;
                    return Ok(Rest)
                }
            }
        }

        fn width(&self) -> usize {
            self.x_range.len()
        }

        fn height(&self) -> usize {
            self.y_range.len()
        }

        fn field(&self, x: usize, y: usize) -> Result<FieldContent, Error> {
            let idx = self.to_vec_idx(x, y)?;
            Ok(self.fields[idx])
        }

        fn to_vec_idx(&self, x: usize, y: usize) -> Result<usize, Error> {
            if !(self.x_range.contains(&x) && self.y_range.contains(&y)) {
                Err(CoordsOutOfBounds(x, y, self.x_range.clone(), self.y_range.clone()))
            } else {
                Ok((y - self.y_range.start) * self.width() + (x - self.x_range.start))
            }
        }
    }

    impl Debug for Map {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for y in self.y_range.clone() {
                for x in self.x_range.clone() {
                    let char = match self.field(x, y).unwrap() {
                        Empty => '.',
                        Rock => '#',
                        RestingSand => 'o'
                    };
                    f.write_char(char)?
                }
                f.write_char('\n')?
            }
            Ok(())
        }
    }
}

mod parse {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::Error::*;
    use super::Error;
    use super::map::*;

    pub(crate) fn parse(path: &str) -> Result<Map, Error> {
        let points = parse_points(path)?;
        let proj = |f: fn(&(usize, usize)) -> &usize| points.iter()
            .flat_map(|ps| ps.iter().map(f))
            .collect::<Vec<_>>();
        let xs = proj(|(x, _)| x);
        let ys = proj(|(_, y)| y);
        // TODO: dedupe
        let xmin = xs.iter().min().ok_or(EmptyPoints)?.to_owned().to_owned().min(500);
        let xmax = xs.iter().max().ok_or(EmptyPoints)?.to_owned().to_owned().max(500);
        let ymin = ys.iter().min().ok_or(EmptyPoints)?.to_owned().to_owned().min(0);
        let ymax = ys.iter().max().ok_or(EmptyPoints)?.to_owned().to_owned().max(0);
        let mut map = Map::new(xmin..(xmax+1), ymin..(ymax+1));

        for line in points {
            let mut ps = line.iter();
            let mut prev = ps.next().ok_or(EmptyPoints)?;
            loop {
                let next = ps.next();
                if next.is_none() {
                    break;
                }
                let next = next.unwrap();
                for x in prev.0.min(next.0)..=(prev.0.max(next.0)) {
                    for y in prev.1.min(next.1)..=(prev.1.max(next.1)) {
                        map.set_field(x, y, FieldContent::Rock)?;
                    }
                }
                prev = next;
            }
        }

        Ok(map)
    }

    fn parse_points(path: &str) -> Result<Vec<Vec<(usize, usize)>>, Error> {
        let mut lines = vec![];
        let file = File::open(path).map_err(|e| IO(e))?;
        for line in BufReader::new(file).lines() {
            let line = line.map_err(|e| IO(e))?;
            let mut points = vec![];
            for s in line.split(" -> ") {
                let mut xy = s.split(",");
                let mut next_num = || xy.next()
                    .ok_or(EmptyCoords(line.to_string()))?
                    .parse::<usize>()
                    .map_err(|e| ParseInt(e));
                let x = next_num()?;
                let y = next_num()?;
                points.push((x, y));
            }
            lines.push(points)
        }
        Ok(lines)
    }
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_count_resting() {
        println!("{}", count_resting().unwrap());
    }
}