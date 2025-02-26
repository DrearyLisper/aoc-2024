use std::iter::zip;

fn parse(content: &String) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in content.split("\n") {
        let row: Vec<&str> = row.split_whitespace().collect();

        left.push(
            row.get(0)
                .expect("Expect 0 element in row")
                .parse()
                .expect("Bad number for 0 element"),
        );

        right.push(
            row.get(1)
                .expect("Expect 1 element in row")
                .parse()
                .expect("Bad number for 1 element"),
        );
    }

    return (left, right);
}

fn part01(content: &String) -> i32 {
    let (mut left, mut right) = parse(content);

    left.sort();
    right.sort();

    let mut result = 0;

    for (a, b) in zip(left, right) {
        result += (a - b).abs()
    }

    result
}

fn part02(content: &String) -> i32 {
    let (mut left, mut right) = parse(content);

    left.sort();
    right.sort();

    let mut result = 0;
    let mut li = 0;
    let mut ri = 0;

    while li < left.len() {
        let l = left[li];

        let mut l_count = 0;
        let mut r_count = 0;

        while ri < right.len() && l >= right[ri] {
            if right[ri] == l {
                r_count += 1;
            }
            ri += 1;
        }

        while li < left.len() && left[li] == l {
            l_count += 1;
            li += 1;
        }

        result += l * l_count * r_count;
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("01")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
