use std::collections::{HashMap, HashSet};

pub struct Input {
    orderings: HashMap<i32, HashSet<i32>>,
    pages: Vec<Vec<i32>>,
}

fn parse(content: &String) -> Input {
    fn parse_orderings(orderings: &str) -> HashMap<i32, HashSet<i32>> {
        let mut result = HashMap::new();

        for row in orderings.split("\n") {
            let row: Vec<&str> = row.split("|").collect();
            let (a, b) = (
                row[0].parse::<i32>().unwrap(),
                row[1].parse::<i32>().unwrap(),
            );
            result.entry(a).or_insert(HashSet::new()).insert(b);
        }

        result
    }

    fn parse_pages(pages: &str) -> Vec<Vec<i32>> {
        let mut result = vec![];

        for row in pages.split("\n") {
            result.push(row.split(",").map(|i| i.parse::<i32>().unwrap()).collect());
        }

        result
    }

    let orderings_and_pages: Vec<&str> = content.split("\n\n").collect();
    let (orderings, pages) = (orderings_and_pages[0], orderings_and_pages[1]);

    return Input {
        orderings: parse_orderings(orderings),
        pages: parse_pages(pages),
    };
}

fn part01(content: &String) -> i32 {
    let input = parse(content);

    let mut result = 0;

    for pages in input.pages {
        let mut good = true;
        for l in 0..pages.len() {
            for r in (l + 1)..pages.len() {
                good = good
                    && match input.orderings.get(&pages[l as usize]) {
                        Some(h) => h.contains(&pages[r as usize]),
                        None => false,
                    }
            }
        }
        if good {
            result += pages[pages.len() / 2];
        }
    }

    result
}

fn part02(content: &String) -> i32 {
    let input = parse(content);

    let mut result = 0;

    for pages in input.pages {
        let mut good = true;

        for l in 0..pages.len() {
            for r in (l + 1)..pages.len() {
                let in_order = match input.orderings.get(&pages[l as usize]) {
                    Some(h) => h.contains(&pages[r as usize]),
                    None => false,
                };
                good = good && in_order;
            }
        }

        if good {
            continue;
        }

        let mut corrected: Vec<i32> = pages.clone();

        for _ in 0..(pages.len() * pages.len()) {
            let mut at_least_on_bad = false;
            for l in 0..pages.len() {
                for r in (l + 1)..pages.len() {
                    let in_order = match input.orderings.get(&corrected[l as usize]) {
                        Some(h) => h.contains(&corrected[r as usize]),
                        None => false,
                    };
                    if !in_order {
                        corrected.swap(l, r);
                        at_least_on_bad = true;
                        break;
                    }
                }
            }
            if !at_least_on_bad {
                break;
            }
        }

        result += corrected[pages.len() / 2];
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("05")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
