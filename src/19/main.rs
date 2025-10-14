use regex::Regex;

#[derive(Debug)]
struct State {
    towels: Vec<String>,
    patterns: Vec<String>,
}

fn parse(content: &String) -> State {
    let mut towels = vec![];
    let mut patterns = vec![];

    for (row, line) in content.lines().enumerate() {
        if row == 0 {
            towels = line.split(", ").map(|i| String::from(i)).collect();
        }

        if row >= 2 {
            patterns.push(String::from(line));
        }
    }

    State {towels, patterns}
}

fn part01(content: &String) -> i32 {
    let state = parse(content);

    let raw_re = format!("^({})+$", state.towels.join("|"));
    let re = Regex::new(&raw_re).unwrap();

    let mut result = 0;
    for pattern in state.patterns {
        if re.is_match(&pattern) {
            result += 1;
        }
    }

    result
}

fn part02(content: &String) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("19")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
