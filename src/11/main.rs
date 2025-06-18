use std::collections::HashMap;

fn parse(content: &String) -> Vec<i32> {
    content
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn count_stones(stone: u128, ticks: i32, memory: &mut HashMap<(u128, i32), u128>) -> u128 {

    if let Some(v) = memory.get(&(stone, ticks)) {
        return *v;
    }

    if ticks == 0 {
        memory.insert((stone, ticks), 1);
        return 1;
    }

    if stone == 0 {
        let c = count_stones(stone + 1, ticks - 1, memory);
        memory.insert((stone, ticks), c);
        return c;
    }

    if stone.to_string().len() % 2 == 0 {
        let stone_str = stone.to_string();

        let left_stone = stone_str[0..stone_str.len() / 2].parse::<u128>().unwrap();
        let right_stone = stone_str[stone_str.len() / 2..].parse::<u128>().unwrap();

        let c = count_stones(left_stone, ticks - 1, memory) 
        + count_stones(right_stone, ticks - 1, memory);

        memory.insert((stone, ticks), c);

        return c;
    }

    return count_stones(stone * 2024, ticks - 1, memory);
}

fn part01(content: &String) -> u128 {
    let stones = parse(content);

    let mut memory: HashMap<(u128, i32), u128> = HashMap::new();

    stones
        .into_iter()
        .map(|x| count_stones(x as u128, 25, &mut memory))
        .sum()
}

fn part02(content: &String) -> u128 {
    let stones = parse(content);

    let mut memory: HashMap<(u128, i32), u128> = HashMap::new();

    stones
        .into_iter()
        .map(|x| {
            count_stones(x as u128, 75, &mut memory)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("11")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
