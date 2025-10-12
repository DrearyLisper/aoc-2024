#[derive(Debug)]
struct State {
    a: i128,
    b: i128,
    c: i128,
    program: Vec<i8>,
    ip: i8,
    halt: bool,
    output: Vec<i8>,
}

fn parse(content: &String) -> State {
    let mut raw = vec![];
    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        raw.push(parts[1]);
    }

    State {
        a: raw[0].parse().unwrap(),
        b: raw[1].parse().unwrap(),
        c: raw[2].parse().unwrap(),
        program: raw[3]
            .split(",")
            .map(|i| i.parse::<i8>().unwrap())
            .collect(),
        ip: 0,
        halt: false,
        output: vec![],
    }
}

fn combo(state: &mut State, operand: i8) -> i128 {
    match operand {
        4 => state.a,
        5 => state.b,
        6 => state.c,
        0..=3 => operand as i128,
        _ => panic!("Wrong combo operand!"),
    }
}

fn adv(state: &mut State, operand: i8) {
    let combo = combo(state, operand);
    state.a = state.a / (2 as i128).pow(combo as u32) as i128;
    state.ip += 2;
}

fn bxl(state: &mut State, operand: i8) {
    state.b = state.b ^ (operand as i128);
    state.ip += 2;
}

fn bst(state: &mut State, operand: i8) {
    let combo = combo(state, operand);
    state.b = (combo % 8) as i128;
    state.ip += 2;
}

fn jnz(state: &mut State, operand: i8) {
    if state.a != 0 {
        state.ip = operand;
    } else {
        state.ip += 2;
    }
}

fn bxc(state: &mut State, _: i8) {
    state.b = state.b ^ state.c;
    state.ip += 2;
}

fn out(state: &mut State, operand: i8) {
    let combo = combo(state, operand);
    state.output.push((combo % 8) as i8);
    state.ip += 2;
}

fn bdv(state: &mut State, operand: i8) {
    let combo = combo(state, operand);
    state.b = state.a / (2 as i128).pow(combo as u32) as i128;
    state.ip += 2;
}

fn cdv(state: &mut State, operand: i8) {
    let combo = combo(state, operand);
    state.c = state.a / (2 as i128).pow(combo as u32) as i128;
    state.ip += 2;
}

fn run(state: &mut State) {
    while !state.halt {
        if state.ip < 0 || state.ip >= state.program.len() as i8 {
            state.halt = true;
            break;
        }

        let opcode = state.program[state.ip as usize];
        let operand = state.program[(state.ip + 1) as usize];

        match opcode {
            0 => adv(state, operand),
            1 => bxl(state, operand),
            2 => bst(state, operand),
            3 => jnz(state, operand),
            4 => bxc(state, operand),
            5 => out(state, operand),
            6 => bdv(state, operand),
            7 => cdv(state, operand),
            x => panic!("Unknown opcode {}", x),
        }
    }
}

fn part01(content: &String) -> String {
    let mut state = parse(content);
    run(&mut state);
    state
        .output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part02(content: &String) -> i128 {
    let raw_a = vec![4, 5, 2, 6, 4, 4, 4, 1, 3, 3, 2, 0, 0, 0, 0, 0];
    let mut a = 0;
    for i in raw_a {
        a *= 8;
        a += i;
    }

    loop {
        let mut state = parse(content);
        state.a = a;
        run(&mut state);
        if state.output == state.program {
            break;
        }
        a += 1;
    }
    a
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("17")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
