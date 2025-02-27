use regex::Regex;

fn part01(content: &String) -> i32 {
    fn parse(content: &String) -> Vec<(i32, i32)> {
        let mut result = vec![];

        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

        for (_, [a, b]) in re.captures_iter(content).map(|c| c.extract()) {
            result.push((
                a.parse().expect("Expect i32"),
                b.parse().expect("Expect i32"),
            ));
        }

        result
    }

    let muls = parse(content);
    muls.iter().map(|(a, b)| a * b).sum()
}

fn part02(content: &String) -> i32 {
    fn parse(content: &String) -> Vec<(i32, i32)> {
        let mut result = vec![];

        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(don't\(\))()|(do\(\))()").unwrap();

        let mut store = true;

        for (_, [a, b]) in re.captures_iter(content).map(|c| c.extract()) {
            if a == "don't()" {
                store = false;
                continue;
            }

            if a == "do()" {
                store = true;
                continue;
            }

            if !store {
                continue;
            }

            result.push((
                a.parse().expect("Expect i32"),
                b.parse().expect("Expect i32"),
            ));
        }

        result
    }

    let muls = parse(content);
    muls.iter().map(|(a, b)| a * b).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("03")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
