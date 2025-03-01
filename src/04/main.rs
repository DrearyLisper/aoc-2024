fn parse(content: &String) -> Vec<Vec<char>> {
    let mut map = vec![];

    for row in content.split("\n") {
        map.push(row.chars().collect());
    }

    map
}

fn part01(content: &String) -> usize {
    fn search(map: &Vec<Vec<char>>, pattern: &Vec<char>) -> Vec<(i32, i32)> {
        fn search_impl(
            point: (i32, i32),
            shift: (i32, i32),
            map: &Vec<Vec<char>>,
            pattern: &Vec<char>,
            index: usize,
        ) -> bool {
            if index == pattern.len() {
                return true;
            }

            let (i, j) = point;
            if i < 0 || (i as usize) >= map.len() {
                return false;
            }
            if j < 0 || (j as usize) >= map[i as usize].len() {
                return false;
            }

            if map[i as usize][j as usize] != pattern[index] {
                return false;
            }

            let (di, dj) = shift;
            let (ni, nj) = (i + di, j + dj);

            if search_impl((ni, nj), shift, map, pattern, index + 1) {
                return true;
            }

            false
        }

        let mut result = vec![];

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let shift = (di, dj);
                        if search_impl((i as i32, j as i32), shift, &map, &pattern, 0) {
                            result.push((i as i32, j as i32));
                        }
                    }
                }
            }
        }

        result
    }

    let map = parse(content);
    let pattern: Vec<char> = "XMAS".chars().collect();
    let matches = search(&map, &pattern);
    matches.len()
}

fn part02(content: &String) -> usize {
    fn search(
        map: &Vec<Vec<char>>,
        pattern: &Vec<char>,
        pattern_reverse: &Vec<char>,
    ) -> Vec<(i32, i32)> {
        fn search_impl(
            point: (i32, i32),
            shift: (i32, i32),
            map: &Vec<Vec<char>>,
            pattern: &Vec<char>,
            index: usize,
        ) -> bool {
            if index == pattern.len() {
                return true;
            }

            let (i, j) = point;
            if i < 0 || (i as usize) >= map.len() {
                return false;
            }
            if j < 0 || (j as usize) >= map[i as usize].len() {
                return false;
            }

            if map[i as usize][j as usize] != pattern[index] {
                return false;
            }

            let (di, dj) = shift;
            let (ni, nj) = (i + di, j + dj);

            if search_impl((ni, nj), shift, map, pattern, index + 1) {
                return true;
            }

            false
        }

        let mut result = vec![];

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if (search_impl((i as i32, j as i32), (1, 1), &map, &pattern, 0)
                    || search_impl((i as i32, j as i32), (1, 1), &map, &pattern_reverse, 0))
                    && (search_impl((i as i32, (j + 2) as i32), (1, -1), &map, &pattern, 0)
                        || search_impl(
                            (i as i32, (j + 2) as i32),
                            (1, -1),
                            &map,
                            &pattern_reverse,
                            0,
                        ))
                {
                    result.push((i as i32, j as i32));
                }
            }
        }

        result
    }

    let map = parse(content);
    let pattern: Vec<char> = "MAS".chars().collect();
    let pattern_reverse: Vec<char> = "SAM".chars().collect();
    let matches = search(&map, &pattern, &pattern_reverse);
    matches.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("04")?;
    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
