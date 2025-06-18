use std::collections::HashSet;

fn parse(content: &String) -> Vec<Vec<char>> {
    let mut result = vec![];

    for line in content.split("\n") {
        result.push(line.chars().collect());
    }

    return result;
}

fn dfs(map: &mut Vec<Vec<char>>, point: (i32, i32)) -> (HashSet<(i32, i32, char)>, u32) {
    let (x, y) = point;
    let plot = map[x as usize][y as usize];

    let mut area = 1;
    let mut perimeter = HashSet::new();

    map[x as usize][y as usize] = plot.to_ascii_lowercase();

    for dx in -1..2 {
        for dy in -1..2 {
            if ((dx + dy) as i32).abs() != 1 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= map.len() as i32 {
                if nx < 0 {
                    perimeter.insert((nx, y, 'b'));
                } else {
                    perimeter.insert((nx, y, 't'));
                }
                continue;
            }

            if ny < 0 || ny >= map[x as usize].len() as i32 {
                if ny < 0 {
                    perimeter.insert((x, ny, 'r'));
                } else {
                    perimeter.insert((x, ny, 'l'));
                }
                continue;
            }

            if map[nx as usize][ny as usize] != plot {
                if map[nx as usize][ny as usize] != plot.to_ascii_lowercase() {
                    let side = match (dx, dy) {
                        (-1, 0) => 'b',
                        (1, 0) => 't',
                        (0, -1) => 'r',
                        (0, 1) => 'l',
                        _ => panic!("Shouldn't happen!"),
                    };
                    perimeter.insert((nx, ny, side));
                }
                continue;
            }

            let (remaining_perimeter, remaining_area) = dfs(map, (nx, ny));

            perimeter.extend(remaining_perimeter);
            area += remaining_area;
        }
    }

    (perimeter, area)
}

fn part01(content: &String) -> u32 {
    let mut map = parse(content);

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].is_ascii_uppercase() {
                let (perimeter, area) = dfs(&mut map, (i as i32, j as i32));
                result += area * perimeter.len() as u32;
            }
        }
    }

    result
}

fn count_sides(perimeter: &HashSet<(i32, i32, char)>) -> u32 {
    let mut result = 0;

    for s in ['l', 'r'] {
        let mut side: Vec<(&i32, &i32)> = perimeter
            .iter()
            .filter(|(_, _, side)| *side == s)
            .map(|(x, y, _)| (y, x))
            .collect();

        side.sort();

        result += 1;
        for i in 1..side.len() {
            if *side[i - 1].0 != *side[i].0 {
                result += 1;
                continue;
            }

            if (side[i - 1].1 + 1) == *side[i].1 {
                continue;
            }

            result += 1;
        }
    }

    for s in ['b', 't'] {
        let mut side: Vec<(&i32, &i32)> = perimeter
            .iter()
            .filter(|(_, _, side)| *side == s)
            .map(|(x, y, _)| (x, y))
            .collect();

        side.sort();

        result += 1;
        for i in 1..side.len() {
            if *side[i - 1].0 != *side[i].0 {
                result += 1;
                continue;
            }

            if (side[i - 1].1 + 1) == *side[i].1 {
                continue;
            }

            result += 1;
        }
    }

    result
}

fn part02(content: &String) -> u32 {
    let mut map = parse(content);

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].is_ascii_uppercase() {
                let (perimeter, area) = dfs(&mut map, (i as i32, j as i32));
                result += area * count_sides(&perimeter);
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("12")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
