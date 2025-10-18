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
        if line.is_empty() {
            break;
        }
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

fn search(state: &State, radius: i32) -> Vec<(i32, Vec<(i32, i32)>)> {
    let mut queue: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();

    let mut seen = HashMap::new();

    let mut shortest_length = 0;

    queue.push(state.end, 0);

    while queue.len() > 0 {
        let ((x, y), length) = queue.pop().unwrap();

        if state.start == (x, y) {
            if shortest_length == 0 {
                shortest_length = length;
            }
        }

        for dx in -1_i32..=1_i32 {
            for dy in -1_i32..=1_i32 {
                if dx.abs() + dy.abs() != 1 {
                    continue;
                }

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

                if seen.contains_key(&(nx, ny)) {
                    continue;
                }

                seen.insert((nx, ny), ((x, y), length));

                queue.push((nx, ny), length - 1);
            }
        }
    }

    let ((mut x, mut y), mut length) = (state.start, shortest_length);
    let mut seen_cheats: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut result: Vec<(i32, Vec<(i32, i32)>)> = vec![(-shortest_length, vec![])];

    while (x, y) != state.end {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx.abs() + dy.abs() <= 1 {
                    continue;
                }
                if dx.abs() + dy.abs() > radius {
                    continue;
                }

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

                let cheat = ((x, y), (nx, ny));

                if seen_cheats.contains(&cheat) {
                    continue;
                }

                seen_cheats.insert(cheat);

                result.push((
                    (length - shortest_length) + dx.abs() + dy.abs()
                        - seen.get(&(nx, ny)).unwrap().1,
                    vec![(x, y), (nx, ny)],
                ));
            }
        }

        ((x, y), length) = *seen.get(&(x, y)).unwrap();
    }

    result
}

fn part01(content: &String) -> i32 {
    let state = parse(content);
    let paths = search(&state, 2);

    let longest = paths.iter().filter(|i| i.1.len() == 0).last().unwrap().0;

    paths
        .iter()
        .filter(|(i, _)| longest - i >= 100)
        .collect::<Vec<_>>()
        .len() as i32
}

fn part02(content: &String) -> i32 {
    let state = parse(content);
    let paths = search(&state, 20);

    let longest = paths.iter().filter(|i| i.1.len() == 0).last().unwrap().0;

    paths
        .iter()
        .filter(|(i, _)| longest - i >= 100)
        .collect::<Vec<_>>()
        .len() as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("20")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
