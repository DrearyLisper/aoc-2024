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

// 0 -- east
// 1 -- south
// 2 -- west
// 3 -- north
fn turn_cost(f: i32, t: i32) -> i32 {
    match (f, t) {
        (0, 0) => 0,
        (0, 2) => 2000,
        (0, _) => 1000,
        (1, 1) => 0,
        (1, 3) => 2000,
        (1, _) => 1000,
        (2, 2) => 0,
        (2, 0) => 2000,
        (2, _) => 1000,
        (3, 3) => 0,
        (3, 1) => 2000,
        (3, _) => 1000,
        _ => panic!("Should never happen"),
    }
}

fn search(state: &State) -> i32 {
    let mut pq: PriorityQueue<(i32, i32, i32), i32> = PriorityQueue::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let (x, y) = state.start;
    let s = (x, y, 0);

    pq.push(s, 0);
    seen.insert((x, y));

    while pq.len() > 0 {
        let ((x, y, d), c) = pq.pop().unwrap();

        if (x, y) == state.end {
            return -c;
        }

        for (dx, dy, dd) in vec![(-1, 0, 3), (1, 0, 1), (0, -1, 2), (0, 1, 0)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= state.map.len() as i32 {
                continue;
            }

            if ny < 0 || ny >= state.map[0].len() as i32 {
                continue;
            }

            if state.map[nx as usize][ny as usize] == '#' {
                continue;
            }

            if seen.contains(&(nx, ny)) {
                continue;
            }

            seen.insert((nx, ny));

            pq.push((nx, ny, dd), c - 1 - turn_cost(d, dd));
        }
    }

    panic!("Can't reach the end point!");
}

fn search_with_path(state: &State) -> i32 {
    let best = search(state);

    let mut pq: PriorityQueue<(i32, i32, i32, Vec<(i32, i32)>), i32> = PriorityQueue::new();
    let mut seen = HashMap::new();

    let (x, y) = state.start;
    let s = (x, y, 0, vec![(x, y)]);

    pq.push(s, 0);
    seen.insert((x, y), 0);

    let mut paths: Vec<Vec<(i32, i32)>> = vec![];

    while pq.len() > 0 {
        let ((x, y, d, path), c) = pq.pop().unwrap();

        if (x, y) == state.end {
            if -c == best {
                paths.push(path.clone());
            }
            continue;
        }

        for (dx, dy, dd) in vec![(-1, 0, 3), (1, 0, 1), (0, -1, 2), (0, 1, 0)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= state.map.len() as i32 {
                continue;
            }

            if ny < 0 || ny >= state.map[0].len() as i32 {
                continue;
            }

            if state.map[nx as usize][ny as usize] == '#' {
                continue;
            }

            let nc = c - 1 - turn_cost(d, dd);

            if seen.contains_key(&(nx, ny)) {
                if (*seen.get(&(nx, ny)).unwrap() - nc) > 1000 {
                    continue;
                }
            } else {
                seen.insert((nx, ny), nc);
            }

            let new_path = vec![path.clone(), vec![(nx, ny)]].concat();

            pq.push((nx, ny, dd, new_path), nc);
        }
    }

    let mut all = paths.concat();

    all.sort();
    all.dedup();

    all.len() as i32
}

fn part01(content: &String) -> i32 {
    let state = parse(content);
    search(&state)
}

fn part02(content: &String) -> i32 {
    let state = parse(content);
    search_with_path(&state)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("16")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
