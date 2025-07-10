use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse(content: &String) -> Vec<Robot> {
    let mut robots = vec![];

    let re = Regex::new(r"p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)").unwrap();

    for line in content.split("\n") {
        let mut robot = Robot {
            position: (0, 0),
            velocity: (0, 0),
        };

        for (_, [p_x, p_y, v_x, v_y]) in re.captures_iter(line).map(|c| c.extract()) {
            robot.position = (p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap());
            robot.velocity = (v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap());
        }

        robots.push(robot);
    }

    robots
}

fn iter(robot: &mut Robot, (w, h): (i32, i32)) {
    robot.position.0 = (robot.position.0 + robot.velocity.0 + w) % w;
    robot.position.1 = (robot.position.1 + robot.velocity.1 + h) % h;
}

fn quadrant(robot: &Robot, (w, h): (i32, i32)) -> Option<i32> {
    if robot.position.0 == (w / 2) {
        return None;
    }

    if robot.position.1 == (h / 2) {
        return None;
    }

    let x = if robot.position.0 < (w / 2) {
        robot.position.0
    } else {
        robot.position.0 - 1
    };
    let y = if robot.position.1 < (h / 2) {
        robot.position.1
    } else {
        robot.position.1 - 1
    };

    Some(x / (w / 2) + 2 * (y / (h / 2)))
}

fn part01(content: &String) -> usize {
    let mut robots = parse(content);

    let wh = (101, 103);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            iter(robot, wh);
        }
    }

    robots
        .iter()
        .map(|r| quadrant(r, wh))
        .fold(HashMap::<i32, usize>::new(), |mut m, q| {
            if let Some(qq) = q {
                *m.entry(qq).or_default() += 1;
            }
            m
        })
        .values()
        .product()
}

fn symmetries(robots: &Vec<Robot>) -> i32 {
    let mut result = 0;

    let avg = robots.iter().map(|r| r.position.0).into_iter().sum::<i32>() / (robots.len() as i32);

    let positions: HashSet<(i32, i32)> =
        HashSet::from_iter(robots.iter().map(|r| (r.position.0, r.position.1)));

    for r in robots {
        if positions.contains(&(r.position.0 + 2 * (avg - r.position.0), r.position.1)) {
            result += 1;
        }
    }

    result
}

fn part02(content: &String) -> i32 {
    let mut robots = parse(content);

    let wh = (101, 103);

    let mut trace: Vec<(i32, i32)> = vec![];

    for i in 0..10000 {
        for robot in robots.iter_mut() {
            iter(robot, wh);
        }
        trace.push((-symmetries(&robots), i));
    }

    trace.sort();

    trace[0].1 + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("14")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
