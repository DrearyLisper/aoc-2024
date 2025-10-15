use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

#[derive(Debug)]
struct State {
    start: (i32, i32),
    end: (i32, i32),
    map: Vec<Vec<char>>,
}

fn parse(content: &String) -> State {
    let lines = content.lines();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];

    for (row, line) in lines.enumerate() {
        if let Some(column) = line.find("S") {
            start = (row as i32, column as i32);
        }
        if let Some(column) = line.find("E") {
            end = (row as i32, column as i32);
        }

        map.push(line.chars().collect());
    }

    State { start, end, map }
}

fn search(state: &State) -> Vec<(i32, i32)> {
    let mut pq: PriorityQueue<((i32, i32), i32), i32> = PriorityQueue::new();
    let mut seen: HashSet<((i32, i32), i32)> = HashSet::new();


    let s = (state.start, 2);
    pq.push(s, 0);

    let mut costs = vec![];

    while pq.len() > 0 {
        let (((x, y), m), c) = pq.pop().unwrap();

        if -c > 9216 {
            continue;
        }

        /*
        if seen.contains(&((x, y), m)) {
            continue;
        }
        seen.insert(((x, y), m));
        */

        let mut nm = m;

        if state.map[x as usize][y as usize] == '#' {
            if m == 0 {
                continue;
            } else {
                nm -= 1;
            }
        }

        if m == 1 {
            nm = 0;
        }

        if (x, y) == state.end {
            if -c <= 9216 {
                costs.push((-c, nm));
            }
            continue;
        }

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= state.map.len() as i32 {
                continue;
            }

            if ny < 0 || ny >= state.map[0].len() as i32 {
                continue;
            }

            pq.push(((nx, ny), nm), c - 1);
        }
    }

    costs
}

fn part01(content: &String) -> i32 {
    let state = parse(content);
    println!("{:?}", search(&state));
    0
}

fn part02(content: &String) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("20")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
