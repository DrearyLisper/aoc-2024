use std::cmp;

#[derive(Debug, Clone)]
struct Block {
    id: Vec<u32>,
    size: Vec<u32>,
    capacity: u32,
}

fn parse(content: &String) -> Vec<Block> {
    let mut result = vec![];
    for (i, c) in content.chars().enumerate() {
        let block = Block {
            id: if i % 2 == 0 {
                vec![(i as u32) / 2]
            } else {
                vec![]
            },
            size: if i % 2 == 0 {
                vec![c.to_digit(10).unwrap()]
            } else {
                vec![]
            },
            capacity: c.to_digit(10).unwrap(),
        };
        result.push(block);
    }
    result
}

fn checksum(blocks: &Vec<Block>) -> u128 {
    let mut i: u128 = 0;
    let mut result: u128 = 0;

    for block in blocks {
        let mut sizes = 0;
        for j in 0..block.id.len() {
            let id = block.id[j];
            let size = block.size[j];
            for _ in 0..size {
                result = result + i * id as u128;
                i = i + 1;
            }

            sizes += size as u128;
        }
        i = i + (block.capacity as u128 - sizes);
    }

    result
}

fn part01(content: &String) -> u128 {
    let mut blocks: Vec<Block> = parse(content);

    let mut l = 0;
    let mut r = blocks.len() - 1;

    while l != r {
        if blocks[l].size.iter().sum::<u32>() == blocks[l].capacity {
            l = l + 1;
            continue;
        }

        if blocks[r].size.iter().sum::<u32>() == 0 {
            r = r - 1;
            continue;
        }

        let how_many: u32 = cmp::min(
            blocks[l].capacity - blocks[l].size.iter().sum::<u32>(),
            blocks[r].size.iter().sum::<u32>(),
        );
        let what: u32 = *blocks[r].id.first().unwrap();

        blocks[l].id.push(what);
        blocks[l].size.push(how_many);

        blocks[r].size[0] = blocks[r].size[0] - how_many;
    }

    checksum(&blocks)
}

fn part02(content: &String) -> u128 {
    let mut blocks: Vec<Block> = parse(content);

    let mut r = blocks.len() - 1;

    while r > 0 {
        if blocks[r].size.iter().sum::<u32>() == 0 {
            r -= 1;
            continue;
        }

        for l in 0..r {
            if blocks[l].size.iter().sum::<u32>() == blocks[l].capacity {
                continue;
            }

            let how_many: u32 = blocks[r].size.iter().sum::<u32>();

            if blocks[l].capacity - blocks[l].size.iter().sum::<u32>() < how_many {
                continue;
            }

            let what: u32 = *blocks[r].id.first().unwrap();

            blocks[l].id.push(what);
            blocks[l].size.push(how_many);

            blocks[r].size.clear();
            blocks[r].id.clear();

            break;
        }

        r -= 1;
    }

    checksum(&blocks)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("09")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
