use std::collections::HashSet;

#[derive(Debug)]
pub struct State {
    map: Vec<Vec<bool>>,
    direction: i32,
    position: (i32, i32),
    trace: HashSet<(i32, i32)>,
    trace_with_direction: HashSet<((i32, i32), i32)>,
    is_over: bool,
    is_looped: bool,
}

fn parse(content: &String) -> State {
    let mut position = (0, 0);
    let mut map: Vec<Vec<bool>> = vec![];
    for (i, row) in content.split("\n").enumerate() {
        let mut map_row: Vec<bool> = vec![];
        for (j, c) in String::from(row).chars().enumerate() {
            let cell_value = match c {
                '#' => true,
                _ => false,
            };
            if c == '^' {
                position = (i as i32, j as i32);
            }
            map_row.push(cell_value);
        }
        map.push(map_row);
    }

    State {
        map,
        direction: 0,
        position,
        trace: HashSet::from_iter(vec![position]),
        trace_with_direction: HashSet::from_iter(vec![(position, 0)]),
        is_over: false,
        is_looped: false,
    }
}

fn step(mut state: State) -> State {
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let (i, j) = state.position;
    let (di, dj) = directions[state.direction as usize];
    let (ni, nj) = (i + di, j + dj);

    if (ni < 0) || (ni >= state.map.len() as i32) {
        state.is_over = true;
        return state;
    }

    if (nj < 0) || (nj >= state.map[ni as usize].len() as i32) {
        state.is_over = true;
        return state;
    }

    if state.map[ni as usize][nj as usize] {
        state.direction = (state.direction + 1) % 4;
        return state;
    }

    state.position = (ni, nj);
    state.trace.insert(state.position);

    if state
        .trace_with_direction
        .contains(&(state.position, state.direction))
    {
        state.is_looped = true;
    }

    state
        .trace_with_direction
        .insert((state.position, state.direction));

    state
}

fn part01(content: &String) -> i32 {
    let mut state = parse(content);
    while !state.is_over {
        state = step(state);
    }
    state.trace.len() as i32
}

fn part02(content: &String) -> i32 {
    let mut state = parse(content);
    while !state.is_over {
        state = step(state);
    }

    let mut result = 0;

    for wall in state.trace {
        let mut walled_state = parse(content);

        if wall == walled_state.position {
            continue;
        }

        walled_state.map[wall.0 as usize][wall.1 as usize] = true;

        while !walled_state.is_over && !walled_state.is_looped {
            walled_state = step(walled_state);
        }

        if walled_state.is_looped {
            result += 1;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("06")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
