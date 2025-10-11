use std::collections::HashMap;

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

fn parse_wide(content: &String) -> State {
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
                position = (row as i32, 2 * column as i32);
            }

            let widening = HashMap::from([
                ('#', vec!['#', '#']),
                ('.', vec!['.', '.']),
                ('@', vec!['@', '.']),
                ('O', vec!['[', ']']),
            ]);
            let wide_line = line
                .chars()
                .map(|c| widening.get(&c).unwrap().clone())
                .flatten()
                .collect();

            warehouse.push(wide_line);
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

fn make_move_wide(state: &mut State, m: char) {
    let (dx, dy) = match m {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Should never happen"),
    };

    let (x, y) = state.position;
    let (nx, ny) = (x + dx, y + dy);

    if nx < 0 || nx >= state.warehouse.len() as i32 {
        return;
    }

    if ny < 0 || ny >= state.warehouse[0].len() as i32 {
        return;
    }

    if state.warehouse[nx as usize][ny as usize] == '#' {
        return;
    }

    if state.warehouse[nx as usize][ny as usize] == '.' {
        state.warehouse[x as usize][y as usize] = '.';
        state.warehouse[nx as usize][ny as usize] = '@';
        state.position = (nx, ny);
        return;
    }

    let (boxes, has_space) = if m == '<' || m == '>' {
        discover_boxes_to_left_or_right(state, (dx, dy))
    } else {
        discover_boxes_to_up_or_down(state, state.position, (dx, dy))
    };

    if !has_space {
        return;
    }

    for b in boxes.iter() {
        let (bx, by) = *b;
        state.warehouse[bx as usize][by as usize] = '.';
        state.warehouse[bx as usize][(by + 1) as usize] = '.';
    }

    for (bx, by) in boxes.iter() {
        state.warehouse[(bx + dx) as usize][(by + dy) as usize] = '[';
        state.warehouse[(bx + dx) as usize][(by + dy + 1) as usize] = ']';
    }

    state.warehouse[x as usize][y as usize] = '.';
    state.warehouse[nx as usize][ny as usize] = '@';
    state.position = (nx, ny);
}

fn discover_boxes_to_left_or_right(
    state: &State,
    direction: (i32, i32),
) -> (Vec<(i32, i32)>, bool) {
    let mut boxes = vec![];
    let mut has_space = false;

    let (mut x, mut y) = state.position;
    let (dx, dy) = direction;

    while y >= 0 && y < state.warehouse[x as usize].len() as i32 {
        let (nx, ny) = (x + dx, y + dy);

        if state.warehouse[nx as usize][ny as usize] == '#' {
            break;
        }

        if state.warehouse[nx as usize][ny as usize] == '.' {
            has_space = true;
            break;
        }

        if state.warehouse[nx as usize][ny as usize] == '[' {
            boxes.push((nx, ny));
            x = nx + dx;
            y = ny + dy;
        } else if state.warehouse[nx as usize][ny as usize] == ']' {
            boxes.push((nx, ny - 1));
            x = nx + dx;
            y = ny + dy;
        }
    }

    (boxes, has_space)
}

fn discover_boxes_to_up_or_down(
    state: &State,
    position: (i32, i32),
    direction: (i32, i32),
) -> (Vec<(i32, i32)>, bool) {
    let (x, y) = position;
    let (dx, dy) = direction;

    if x < 0 || x >= state.warehouse.len() as i32 {
        return (vec![], false);
    }

    let (nx, ny) = (x + dx, y + dy);

    if state.warehouse[nx as usize][ny as usize] == '#' {
        return (vec![], false);
    }

    if state.warehouse[nx as usize][ny as usize] == '.' {
        return (vec![], true);
    }

    if state.warehouse[nx as usize][ny as usize] == '[' {
        let (lboxes, lhas_space) = discover_boxes_to_up_or_down(state, (nx, ny), direction);
        let (rboxes, rhas_space) = discover_boxes_to_up_or_down(state, (nx, ny + 1), direction);

        let mut boxes: Vec<(i32, i32)> = vec![vec![(nx, ny)], lboxes, rboxes]
            .into_iter()
            .flatten()
            .collect();

        boxes.sort();
        boxes.dedup();

        let has_space = lhas_space && rhas_space;
        return (boxes, has_space);
    } else {
        let (lboxes, lhas_space) = discover_boxes_to_up_or_down(state, (nx, ny), direction);
        let (rboxes, rhas_space) = discover_boxes_to_up_or_down(state, (nx, ny - 1), direction);

        let mut boxes: Vec<(i32, i32)> = vec![vec![(nx, ny - 1)], lboxes, rboxes]
            .into_iter()
            .flatten()
            .collect();

        boxes.sort();
        boxes.dedup();

        let has_space = lhas_space && rhas_space;
        return (boxes, has_space);
    }
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
    let mut state = parse_wide(content);

    for m in state.moves.clone() {
        make_move_wide(&mut state, m);
    }

    let mut result = 0;

    for (x, row) in state.warehouse.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == '[' {
                result += (x as i32) * 100 + (y as i32);
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("15")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
