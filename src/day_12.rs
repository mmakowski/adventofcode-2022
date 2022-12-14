use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use Error::*;

// directed graph: each node holds a list of next nodes
type Node = usize;
type Graph = Vec<Vec<Node>>;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    EmptyQueue,
    Impossible
}

fn shortest_path() -> Result<u16, Error> {
    let (graph, start, end) = parse_graph("input-12.txt")?;
    // BFS
    let mut fwd_queue = VecDeque::from([start]);
    let mut distance = vec![-1; graph.len()];
    distance[start] = 0;
    // TODO: search from both ends; requires easy navigation of back edges
    loop {
        let v1 = fwd_queue.pop_front().ok_or(EmptyQueue)?;
        for i in 0..graph[v1].len() {
            let v2 = graph[v1][i];
            if distance[v2] == -1 {
                distance[v2] = distance[v1] + 1;
                if v2 == end {
                    return Ok(distance[v2] as u16)
                }
                fwd_queue.push_back(v2)
            }
        }
    }
}

fn parse_graph(path: &str) -> Result<(Graph, Node, Node), Error> {
    let (chars, width) = read_chars(path)?;
    let mut graph = vec![];
    let mut start = 0;
    let mut end = 0;
    for i in 0..chars.len() {
        let mut next_nodes = vec![];
        if chars[i] == 'S' {
            start = i
        } else if chars[i] == 'E' {
            end = i
        }
        if i >= width && can_move(chars[i], chars[i - width]) {
            next_nodes.push(i - width)
        }
        if i < chars.len() - width && can_move(chars[i], chars[i + width]) {
            next_nodes.push(i + width)
        }
        if i % width != 0 && can_move(chars[i], chars[i - 1]) {
            next_nodes.push(i - 1)
        }
        if i % width != (width - 1) && can_move(chars[i], chars[i + 1]) {
            next_nodes.push(i + 1)
        }
        graph.push(next_nodes)
    }
    Ok((graph, start, end))
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
    fn print_shortest_path() {
        println!("{}", shortest_path().unwrap());
    }
}
