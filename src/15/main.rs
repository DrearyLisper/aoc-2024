use core::error;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    position: (i32, i32),
    warehouse: Vec<Vec<char>>,
    moves: Vec<char>,
}

fn parse(content: &String) -> State {
    let lines = content.lines();

    let mut position = (0, 0);
    let mut warehouse = vec![];
    let mut moves: Vec<char> = vec![];
    let mut in_map = true;

    for (row, line) in lines.enumerate() {
        if line.len() == 0 {
            in_map = false;
            continue;
        }

        if in_map {
            if let Some(column) = line.find("@") {
                position = (row as i32, column as i32);
            }

            warehouse.push(line.chars().collect());
        } else {
            moves.append(&mut line.chars().collect());
        }
    }

    State {
        position,
        warehouse,
        moves,
    }
}

fn make_move(state: &mut State, m: char) {
    let (dx, dy) = match m {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Should never happen"),
    };

    let mut boxes = 0;
    let (mut x, mut y) = state.position;

    loop {
        let (nx, ny) = (x + dx, y + dy);

        if nx < 0 || nx >= state.warehouse.len() as i32 {
            break;
        }

        if ny < 0 || ny >= state.warehouse[0].len() as i32 {
            break;
        }

        state.warehouse[x as usize][y as usize] = '.';

        if state.warehouse[nx as usize][ny as usize] == 'O' {
            boxes += 1;
        }

        if state.warehouse[nx as usize][ny as usize] == '#' {
            break;
        }

        (x, y) = (nx, ny);

        if state.warehouse[nx as usize][ny as usize] == '.' {
            break;
        }
    }

    while boxes > 0 {
        state.warehouse[x as usize][y as usize] = 'O';
        (x, y) = (x - dx, y - dy);
        boxes -= 1;
    }
    state.warehouse[x as usize][y as usize] = '@';

    state.position = (x, y);
}

fn part01(content: &String) -> i32 {
    let mut state = parse(content);

    for m in state.moves.clone() {
        make_move(&mut state, m);
    }

    let mut result = 0;

    for (x, row) in state.warehouse.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'O' {
                result += (x as i32) * 100 + (y as i32);
            }
        }
    }

    result
}

fn part02(content: &String) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("15")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
