use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use Error::*;

// directed graph: each node holds a list of previous nodes
type Node = usize;
type Graph = Vec<Vec<Node>>;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    EmptyQueue,
}

fn shortest_start_end() -> Result<u16, Error> {
    let (graph, start, end, _floor) = parse_graph("input-12.txt")?;
    let stop_nodes = HashSet::from([start]);
    shortest_path(&graph, end, &stop_nodes)
}

fn shortest_floor_end() -> Result<u16, Error> {
    let (graph, _start, end, floor) = parse_graph("input-12.txt")?;
    let mut stop_nodes = HashSet::new();
    for i in 0..floor.len() {
        if floor[i] {
            stop_nodes.insert(i);
        }
    }
    shortest_path(&graph, end, &stop_nodes)
}

fn shortest_path(graph: &Graph, end: Node, stop_nodes: &HashSet<Node>) -> Result<u16, Error> {
    // BFS
    let mut queue = VecDeque::from([end]);
    let mut distance = vec![-1; graph.len()];
    distance[end] = 0;
    loop {
        let v1 = queue.pop_front().ok_or(EmptyQueue)?;
        for i in 0..graph[v1].len() {
            let v2 = graph[v1][i];
            if distance[v2] == -1 {
                distance[v2] = distance[v1] + 1;
                if stop_nodes.contains(&v2) {
                    return Ok(distance[v2] as u16)
                }
                queue.push_back(v2)
            }
        }
    }
}

fn parse_graph(path: &str) -> Result<(Graph, Node, Node, Vec<bool>), Error> {
    let (chars, width) = read_chars(path)?;
    let mut graph = vec![];
    let mut floor = vec![false; chars.len()];
    let mut start = 0;
    let mut end = 0;
    for i in 0..chars.len() {
        let mut prev_nodes = vec![];
        if chars[i] == 'S' {
            start = i;
            floor[i] = true
        } else if chars[i] == 'E' {
            end = i
        } else if chars[i] == 'a' {
            floor[i] = true
        }
        if i >= width && can_move(chars[i - width], chars[i]) {
            prev_nodes.push(i - width)
        }
        if i < chars.len() - width && can_move(chars[i + width], chars[i]) {
            prev_nodes.push(i + width)
        }
        if i % width != 0 && can_move(chars[i - 1], chars[i]) {
            prev_nodes.push(i - 1)
        }
        if i % width != (width - 1) && can_move(chars[i + 1], chars[i]) {
            prev_nodes.push(i + 1)
        }
        graph.push(prev_nodes)
    }
    Ok((graph, start, end, floor))
}

fn can_move(c1: char, c2: char) -> bool {
    height(c2) - height(c1) <= 1
}

fn height(c: char) -> i8 {
    match c {
        'S' => 'a' as i8,
        'E' => 'z' as i8,
        _ => c as i8
    }
}

fn read_chars(path: &str) -> Result<(Vec<char>, usize), Error> {
    let mut chars = vec![];
    let mut width = 0;
    let file = File::open(path).map_err(|e| IO(e))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| IO(e))?;
        width = line.len();
        chars.append(line.chars().collect::<Vec<_>>().as_mut())
    }
    Ok((chars, width))
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn print_shortest_start_end() {
        println!("{}", shortest_start_end().unwrap());
    }

    #[test]
    fn print_shortest_floor_end() {
        println!("{}", shortest_floor_end().unwrap());
    }
}
