use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn parse(content: &String, offset: u64) -> Vec<Machine> {
    fn parse_chunk(chunk: Vec<&str>, offset: u64) -> Machine {
        let re_a = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let re_b = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let re_prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

        let mut machine = Machine {
            a: (0, 0),
            b: (0, 0),
            prize: (0, 0),
        };

        for (_, [x, y]) in re_a.captures_iter(chunk[0]).map(|c| c.extract()) {
            machine.a = (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        }

        for (_, [x, y]) in re_b.captures_iter(chunk[1]).map(|c| c.extract()) {
            machine.b = (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        }

        for (_, [x, y]) in re_prize.captures_iter(chunk[2]).map(|c| c.extract()) {
            machine.prize = (x.parse::<u64>().unwrap() + offset, y.parse::<u64>().unwrap() + offset)
        }

        machine
    }

    content
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|c| c.iter().map(|i| *i).collect::<Vec<&str>>())
        .map(|c| parse_chunk(c, offset))
        .collect()
}

fn find(machine: &Machine) -> Option<u64> {
    let mut result: Vec<u64> = vec![];

    for i in 0..100 {
        for j in 0..100 {
            let x_units = machine.a.0 * i + machine.b.0 * j;
            let y_units = machine.a.1 * i + machine.b.1 * j;

            //println!("{i} {j}");

            if x_units == machine.prize.0 && y_units == machine.prize.1 {
                result.push(3 * i + j);
            }
        }
    }

    result.iter().min().copied()
}

fn part01(content: &String) -> u64 {
    let machines = parse(content, 0);

    machines
        .iter()
        .map(find)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .sum()
}

fn find_quick(machine: &Machine) -> Option<u64> {
    let n = (machine.prize.0 as i128) * (machine.b.1 as i128)
                 - (machine.b.0 as i128) * (machine.prize.1 as i128);
    let d = (machine.a.0 as i128) * (machine.b.1 as i128)
                 - (machine.a.1 as i128) * (machine.b.0 as i128);


    if n % d != 0 {
        return None;
    }

    let a = (n / d) as u64;

    let n = (machine.prize.1 as i128) - (machine.a.1 as i128) * (a as i128);
    let d = machine.b.1 as i128;

    if n % d != 0 {
        return None;
    }

    let b = (n / d) as u64;

    Some(a * 3 + b)
}

fn part02(content: &String) -> u64 {
   let machines = parse(content, 10000000000000);

    machines
        .iter()
        .map(find_quick)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("13")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
