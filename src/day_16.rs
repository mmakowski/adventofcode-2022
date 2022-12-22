use std::collections::{HashMap, HashSet, VecDeque};
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

fn max_pressure() -> Result<u64, Error> {
    let (graph, start) = parse("input-16.txt")?;
    let mut free_locs = graph.nodes();
    free_locs.remove(start.as_str());
    let max = max_pressure_p(0, 0, &start, free_locs, &graph, vec![&start])?;
    Ok(max)
}

fn max_pressure_p(time: u32, acc: u64, loc: &String, remaining_locs: HashSet<String>, graph: &Graph, prefix: Vec<&String>) -> Result<u64, Error> {
    const TOTAL_TIME: u32 = 30;
    if remaining_locs.is_empty() {
        //println!("{acc}: {prefix:?}");
        return Ok(acc)
    }
    let mut totals = HashSet::new();
    for next_loc in &remaining_locs {
        let mut next_remaining_locs = remaining_locs.clone();
        next_remaining_locs.remove(next_loc);
        let time_diff = graph.move_time(loc, next_loc) + 1;
        let next_time = time + time_diff;
        if next_time < TOTAL_TIME {
            let next_acc = acc + graph.flow_rate(next_loc) * (TOTAL_TIME - next_time) as u64;
            let mut next_prefix = prefix.clone();
            next_prefix.push(next_loc);
            let path_max = max_pressure_p(next_time, next_acc, next_loc, next_remaining_locs, graph, next_prefix)?;
            totals.insert(path_max);
            //println!("{loc}:{time} -> {next_loc}:{next_time}.. = {path_max}")
        }
    }
    if let Some(max) = totals.iter().max() {
        Ok(max.to_owned())
    } else {
        //println!("{acc}: {prefix:?}");
        Ok(acc)
    }
}

struct Graph {
    move_times: HashMap<String, HashMap<String, u32>>,
    flow_rates: HashMap<String, u64>
}

impl Graph {
    pub fn new(move_times: HashMap<String, HashMap<String, u32>>, flow_rates: HashMap<String, u64>) -> Self {
        Graph { move_times, flow_rates }
    }

    pub fn nodes(&self) -> HashSet<String> {
        HashSet::from_iter(self.move_times.keys().map(|s| s.to_owned()))
    }

    pub fn move_time(&self, from: &String, to: &String) -> u32 {
        self.move_times[from][to]
    }

    pub fn flow_rate(&self, node: &String) -> u64 {
        self.flow_rates[node]
    }
}

fn parse(path: &str) -> Result<(Graph, String), Error> {
    let mut flows = HashMap::new();
    let mut edges = HashMap::new();
    let re = Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? ([\s\w,]+)").unwrap();
    let file = File::open(path).map_err(|e| IO(e))?;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        let cap = re.captures(&line[..]).ok_or(MalformedLine(line.to_string()))?;
        let node = cap[1].to_string();
        let flow = cap[2].parse::<u64>()
            .map_err(ParseInt)?;
        let next_nodes = cap[3].split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        flows.insert(node.clone(), flow);
        edges.insert(node.clone(), next_nodes);
    }
    let start_node = "AA".to_string();
    let mut interesting_nodes = vec![&start_node];
    for (node, flow) in &flows {
        if flow > &0 {
            interesting_nodes.push(node);
        }
    }
    let move_times = calc_move_times(interesting_nodes, edges);
    //println!("{move_times:?}");
    Ok((Graph::new(move_times, flows), start_node))
}

fn calc_move_times(nodes: Vec<&String>, edges: HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, u32>> {
    let mut result = HashMap::new();
    for node in nodes.clone() {
        let node_times = calc_move_times_from_node(node, &nodes, &edges);
        result.insert(node.to_owned(), node_times);
    }
    result
}

fn calc_move_times_from_node(node: &String, nodes: &Vec<&String>, edges: &HashMap<String, Vec<String>>) -> HashMap<String, u32> {
    let mut visited= HashSet::from([node]);
    let mut queue = VecDeque::from([node]);
    let mut result = HashMap::new();
    result.insert(node.to_owned(), 0);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let curr_dist = result[curr];
        for next in &edges[curr] {
            if !visited.contains(next) {
                visited.insert(next);
                result.insert(next.to_owned(), curr_dist + 1);
                queue.push_back(next);
            }
        }
    }
    for (k, v) in result.clone() {
        if !nodes.contains(&&k) || v == 0 {
            result.remove(&k);
        }
    }
    result
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_max_pressure() {
        println!("{}", max_pressure().unwrap());
    }
}