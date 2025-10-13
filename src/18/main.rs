use priority_queue::PriorityQueue;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    start: (i32, i32),
    end: (i32, i32),
    map: Vec<Vec<char>>,
    bytes: Vec<String>
}

fn parse(content: &String, size: (i32, i32), n: i32) -> State {
    let lines = content.lines();

    let start: (i32, i32) = (0, 0);
    let end: (i32, i32) = (size.0 - 1, size.1 - 1);
    let mut map: Vec<Vec<char>> = vec![vec!['.'; size.0 as usize]; size.1 as usize];
    let mut bytes = vec![];

    for (row, line) in lines.enumerate() {
        if row >= n as usize {
            break;
        }

        let coordinates = line
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>();

        map[coordinates[1]][coordinates[0]] = '#';

        bytes.push(String::from(line));
    }

    State { start, end, map, bytes}
}

fn search(state: &State) -> Option<i32> {
    let mut pq: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    pq.push(state.start, 0);
    seen.insert(state.start);

    while pq.len() > 0 {
        let ((x, y), c) = pq.pop().unwrap();

        if (x, y) == state.end {
            return Some(-c);
        }

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= state.map[0].len() as i32 {
                continue;
            }

            if ny < 0 || ny >= state.map.len() as i32 {
                continue;
            }

            if state.map[ny as usize][nx as usize] == '#' {
                continue;
            }

            if seen.contains(&(nx, ny)) {
                continue;
            }

            seen.insert((nx, ny));

            pq.push((nx, ny), c - 1);
        }
    }

    None
}

fn part01(content: &String) -> i32 {
    let state = parse(content, (71, 71), 1024);
    match search(&state) {
        Some(c) => c,
        None => panic!("Should never happen!")
    }
}

fn part02(content: &String) -> String {
    let state = parse(content, (71, 71), 1000000);
    let (mut l, mut r) = (0, (state.bytes.len() - 1) as i32);

    while (r - l) > 1 {
        let m = (r + l) / 2;
        let state = parse(content, (71, 71), m);

        match search(&state) {
            Some(_) => l = m,
            None => r = m,
        }
    }
    return state.bytes[(r - 1) as usize].clone();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("18")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
