use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Input {
    field: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

fn parse(content: &String) -> Input {
    let mut field: Vec<Vec<char>> = vec![];
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, row) in content.lines().enumerate() {
        let mut vec_row = vec![];
        for (j, c) in row.chars().enumerate() {
            vec_row.push(c);
            if c != '.' {
                antennas.entry(c).or_default().push((i as i32, j as i32));
            }
        }
        field.push(vec_row);
    }

    Input { field, antennas }
}

fn part01(content: &String) -> usize {
    fn solve(input: &Input) -> Vec<(i32, i32)> {
        let mut result = HashSet::new();

        for (_, antennas) in &input.antennas {
            for a in antennas {
                for b in antennas {
                    if a == b {
                        continue;
                    }

                    let id = a.0 - b.0;
                    let jd = a.1 - b.1;

                    let ni = b.0 - id;
                    let nj = b.1 - jd;

                    if ni < 0 || ni >= (input.field.len() as i32) {
                        continue;
                    }

                    if nj < 0 || nj >= (input.field[ni as usize].len() as i32) {
                        continue;
                    }

                    result.insert((ni, nj));
                }
            }
        }

        result.into_iter().collect()
    }

    let input = parse(content);
    let antinodes = solve(&input);

    antinodes.len()
}

fn part02(content: &String) -> usize {
    fn solve(input: &Input) -> Vec<(i32, i32)> {
        let mut result = HashSet::new();

        for (_, antennas) in &input.antennas {
            for a in antennas {
                for b in antennas {
                    if a == b {
                        continue;
                    }

                    let id = a.0 - b.0;
                    let jd = a.1 - b.1;

                    for s in (-50)..(50) {
                        let ni = b.0 + s * id;
                        let nj = b.1 + s * jd;

                        if ni < 0 || ni >= (input.field.len() as i32) {
                            continue;
                        }

                        if nj < 0 || nj >= (input.field[ni as usize].len() as i32) {
                            continue;
                        }

                        result.insert((ni, nj));
                    }
                }
            }
        }

        result.into_iter().collect()
    }

    let input = parse(content);
    let antinodes = solve(&input);

    antinodes.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("08")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
