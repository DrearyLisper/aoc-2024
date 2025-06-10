use std::collections::HashSet;

fn parse(content: &String) -> Vec<Vec<u32>> {
    let mut grid = vec![];

    for line in content.split("\n") {
        let mut row = vec![];

        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }

        grid.push(row);
    }

    grid
}

fn part01(content: &String) -> usize {
    let grid = parse(content);

    fn search(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> HashSet<(i32, i32)> {
        if grid[x as usize][y as usize] == 9 {
            return HashSet::from([(x, y)]);
        }

        let mut result: HashSet<(i32, i32)> = HashSet::new();

        for dx in -1..2 {
            for dy in -1..2 {
                let nx = x + dx;
                let ny = y + dy;

                if (dx + dy).abs() != 1 {
                    continue;
                }

                if nx < 0 || nx >= grid.len() as i32 {
                    continue;
                }

                if ny < 0 || ny >= grid[x as usize].len() as i32 {
                    continue;
                }

                if grid[nx as usize][ny as usize] - grid[x as usize][y as usize] != 1 {
                    continue;
                }

                for i in search(grid, nx, ny) {
                    result.insert(i);
                }
            }
        }

        return result;
    }

    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                result += search(&grid, x as i32, y as i32).len();
            }
        }
    }

    result
}

fn part02(content: &String) -> i32 {
    let grid = parse(content);

    fn search(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> i32 {
        if grid[x as usize][y as usize] == 9 {
            return 1;
        }

        let mut result = 0;

        for dx in -1..2 {
            for dy in -1..2 {
                let nx = x + dx;
                let ny = y + dy;

                if (dx + dy).abs() != 1 {
                    continue;
                }

                if nx < 0 || nx >= grid.len() as i32 {
                    continue;
                }

                if ny < 0 || ny >= grid[x as usize].len() as i32 {
                    continue;
                }

                if grid[nx as usize][ny as usize] - grid[x as usize][y as usize] != 1 {
                    continue;
                }

                result += search(grid, nx, ny);
            }
        }

        return result;
    }

    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                result += search(&grid, x as i32, y as i32);
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("10")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
