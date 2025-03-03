fn parse(content: &String) -> Vec<(u64, Vec<u64>)> {
    let mut result = vec![];

    for row in content.split("\n") {
        let fields: Vec<&str> = row.split(": ").collect();

        let total: u64 = fields[0].parse().unwrap();
        let parts: Vec<u64> = fields[1]
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        result.push((total, parts));
    }

    result
}

fn part01(content: &String) -> u64 {
    fn solve(total: u64, parts: &Vec<u64>) -> bool {
        fn solve_impl(total: u64, parts: &Vec<u64>, current: u64, index: usize) -> bool {
            if current > total {
                return false;
            }

            if current == total && index == parts.len() {
                return true;
            }

            if index >= parts.len() {
                return false;
            }

            return solve_impl(total, parts, current * parts[index], index + 1)
                || solve_impl(total, parts, current + parts[index], index + 1);
        }
        solve_impl(total, parts, parts[0], 1)
    }

    let rows = parse(content);
    let mut result: u64 = 0;

    for row in rows.iter() {
        let (total, parts) = row;
        if solve(*total, parts) {
            result += *total;
        }
    }

    result
}

fn part02(content: &String) -> u64 {
    fn solve(total: u64, parts: &Vec<u64>) -> bool {
        fn solve_impl(total: u64, parts: &Vec<u64>, current: u64, index: usize) -> bool {
            if current > total {
                return false;
            }

            if current == total && index == parts.len() {
                return true;
            }

            if index >= parts.len() {
                return false;
            }

            let next_part = parts[index];

            return solve_impl(total, parts, current * next_part, index + 1)
                || solve_impl(total, parts, current + next_part, index + 1)
                || solve_impl(
                    total,
                    parts,
                    format!("{current}{next_part}").parse::<u64>().unwrap(),
                    index + 1,
                );
        }
        solve_impl(total, parts, parts[0], 1)
    }

    let rows = parse(content);
    let mut result: u64 = 0;

    for row in rows.iter() {
        let (total, parts) = row;
        if solve(*total, parts) {
            result += *total;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("07")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
