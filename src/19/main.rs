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

#[derive(Debug)]
struct Trie {
    children:  [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: core::array::from_fn(|_| None),
            end: false,
        }
    }

    pub fn add(&mut self, word: &str) {
        let mut trie = self;
        for c in word.chars() {
            let index: u32 = <char as Into<u32>>::into(c) - <char as Into<u32>>::into('a');

            if let None = trie.children[index as usize] {
                trie.children[index as usize] = Some(Box::new(Trie::new()));
            }

            trie = trie.children[index as usize].as_mut().unwrap();
        }
        trie.end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut trie = self;
        for c in word.chars() {
            let index: u32 = <char as Into<u32>>::into(c) - <char as Into<u32>>::into('a');

            if let None = trie.children[index as usize] {
                return false;
            }

            trie = trie.children[index as usize].as_ref().unwrap();
        }

        return trie.end;
    }
}

fn count(trie: &Trie, word: &str) -> i128 {
    let n = word.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for j in 0..i {
            if trie.search(&word[j..i]) {
                dp[i] += dp[j];
            }
        }
    }
    
    dp[n]
}

fn part02(content: &String) -> i128 {
    let state = parse(content);

    let mut trie = Trie::new();

    let mut result: i128 = 0;

    for towel in state.towels {
        trie.add(&towel);
    }

    for pattern in state.patterns {
        result += count(&trie, &pattern);
    }

   result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("19")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
