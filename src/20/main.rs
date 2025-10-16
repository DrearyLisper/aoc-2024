use std::{
    cell::{Cell, RefCell},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

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

fn search(
    state: &State,
    (x, y): (i32, i32),
    seen: &mut HashMap<(i32, i32), i32>,
    cheat: Vec<(i32, i32)>,
    length: i32,
) -> Vec<(i32, Vec<(i32, i32)>)> {
    if state.end == (x, y) {
        return vec![(length as i32, cheat)];
    }

    let mut result = vec![];

    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = (x + dx, y + dy);

        if nx < 0 || nx >= state.map.len() as i32 {
            continue;
        }

        if ny < 0 || ny >= state.map[0].len() as i32 {
            continue;
        }

        if seen.contains_key(&(nx, ny)) {
            if length >= (*seen.get(&(nx, ny)).unwrap() + 100) {
                continue;
            } else if length < (*seen.get(&(nx, ny)).unwrap() + 100) {
                seen.insert((x, y), length);
            }
        } else {
            seen.insert((x, y), length);
        }

        if state.map[nx as usize][ny as usize] == '#' {
            if cheat.len() < 1 {
                let mut ncheat = cheat.clone();
                ncheat.push((nx, ny));

                let mut nresult = search(state, (nx, ny), seen, ncheat, length + 1);

                result.append(&mut nresult);
            } else {
                continue;
            }
        } else {
            let mut ncheat = cheat.clone();
            if cheat.len() == 1 {
                ncheat.push((nx, ny));
            }

            let mut nresult = search(state, (nx, ny), seen, ncheat, length + 1);

            result.append(&mut nresult);
        }
    }

    result
}

#[derive(Clone, Eq, PartialEq)]
struct SearchState {
    point: (i32, i32),
    cheat: Vec<(i32, i32)>,
    seen: Rc<RefCell<HashSet<(i32, i32)>>>,
    length: i32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search2(state: &State) -> Vec<(i32, Vec<(i32, i32)>)> {
    let mut queue = BinaryHeap::new();

    let search_state = SearchState {
        point: state.start,
        cheat: vec![],
        seen: Rc::new(RefCell::new(HashSet::new())),
        length: 0,
    };

    queue.push(search_state);

    let mut result = vec![];

    while queue.len() > 0 {
        let mut search_state = queue.pop().unwrap();

        let (x, y) = search_state.point;
        let length = search_state.length;
        let cheat = &search_state.cheat;

        let mut seen = search_state.seen.borrow_mut();

        //println!("{}", length);

        if state.end == (x, y) {
            result.push((length, cheat.clone()));

            if cheat.len() == 0 {
                break;
            }
        }

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= state.map.len() as i32 {
                continue;
            }

            if ny < 0 || ny >= state.map[0].len() as i32 {
                continue;
            }

            if seen.contains(&(nx, ny)) {
                continue;
            }

            if state.map[nx as usize][ny as usize] == '#' {
                if cheat.len() < 1 {
                    let mut nseen = (*seen).clone();
                    nseen.insert((nx, ny));

                    let mut ncheat = cheat.clone();
                    ncheat.push((nx, ny));

                    let search_state = SearchState {
                        point: (nx, ny),
                        cheat: ncheat,
                        seen: Rc::new(RefCell::new(nseen)),
                        length: length - 1,
                    };

                    queue.push(search_state);
                } else {
                    continue;
                }
            } else {
                seen.insert((nx, ny));

                let mut ncheat = cheat.clone();
                if cheat.len() == 1 {
                    ncheat.push((nx, ny));
                }

                let search_state = SearchState {
                    point: (nx, ny),
                    cheat: ncheat,
                    seen: search_state.seen.clone(),
                    length: length - 1,
                };

                queue.push(search_state);
            }
        }
    }

    result
}

fn part01(content: &String) -> i32 {
    let state = parse(content);
    let mut paths = search2(&state);

    println!("{:?}", paths);
    //println!("{:?}", paths.iter().map(|i| i.0).collect::<Vec<_>>());

    let longest = paths.iter().filter(|i| i.1.len() == 0).last().unwrap().0;

    paths
        .iter()
        .filter(|(i, _)| i - longest >= 100)
        .collect::<Vec<_>>()
        .len() as i32
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
