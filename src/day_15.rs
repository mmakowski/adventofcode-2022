use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

use regex::Regex;

use Error::*;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    MalformedLine(String),
    ParseInt(ParseIntError)
}

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Clone, Copy, Debug)]
struct RowInterval {
    pub start_x: i64,
    pub end_x: i64
}

impl RowInterval {
    fn len(&self) -> u64 {
        assert!(self.start_x <= self.end_x);
        (self.end_x - self.start_x) as u64
    }

    fn combine_if_overlap(&self, other: &Self) -> Option<Self> {
        assert!(self.start_x <= self.end_x);
        assert!(other.start_x <= other.end_x);
        if self.start_x >= other.start_x && self.start_x <= other.end_x ||
            other.start_x >= self.start_x && other.start_x <= self.end_x {
            Some(RowInterval {
                start_x: self.start_x.min(other.start_x),
                end_x: self.end_x.max(other.end_x)
            })
        } else {
            None
        }
    }
}

struct SensorBeacon {
    pub sensor: Point,
    pub closest_beacon: Point
}

fn count_points_with_no_beacon() -> Result<u64, Error> {
    let sensors = parse("input-15.txt")?;
    let mut raw_intervals = sensors.iter()
        .filter_map(|s| int_row_no_sensor(s, 2000000))
        .collect::<Vec<_>>();
    // combine overlapping
    raw_intervals.sort_by_key(|i| i.start_x);
    let mut combined_intervals = vec![];
    let mut prev = raw_intervals[0];
    for i in 1..raw_intervals.len() {
        let curr = &raw_intervals[i];
        if let Some(combined) = prev.combine_if_overlap(curr) {
            prev = combined
        } else {
            combined_intervals.push(prev);
            prev = curr.clone()
        }
    }
    combined_intervals.push(prev);
    // calc sum of lengths
    let sum = combined_intervals.iter()
        .map(|i| i.len())
        .sum();
    Ok(sum)
}

fn parse(path: &str) -> Result<Vec<SensorBeacon>, Error> {
    let mut result = vec![];
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let file = File::open(path).map_err(|e| IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        let cap = re.captures(&line[..]).ok_or(MalformedLine(line.to_string()))?;
        let num = |i: usize| cap[i].parse::<i64>().map_err(|e| ParseInt(e));
        result.push(SensorBeacon {
            sensor: Point { x: num(1)?, y: num(2)? },
            closest_beacon: Point { x: num(3)?, y: num(4)? }
        })
    }
    Ok(result)
}

fn int_row_no_sensor(sb: &SensorBeacon, y: i64) -> Option<RowInterval> {
    let beacon_dist = sb.closest_beacon.x.abs_diff(sb.sensor.x) +
        sb.closest_beacon.y.abs_diff(sb.sensor.y);
    let y_dist = y.abs_diff(sb.sensor.y);
    if y_dist <= beacon_dist {
        Some(RowInterval {
            start_x: sb.sensor.x - (beacon_dist - y_dist) as i64,
            end_x: sb.sensor.x + (beacon_dist - y_dist) as i64
        })
    } else {
        None
    }
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_count_points_with_no_beacon() {
        println!("{}", count_points_with_no_beacon().unwrap());
    }
}