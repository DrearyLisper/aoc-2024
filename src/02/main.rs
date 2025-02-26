fn parse(content: &String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for row in content.split("\n") {
        result.push(
            row.split_whitespace()
                .map(|i| -> i32 { i.parse().expect("Not a integer") })
                .collect(),
        );
    }

    result
}

fn part01(content: &String) -> usize {
    let reports = parse(content);
    fn check(report: &Vec<i32>) -> bool {
        if report.len() <= 1 {
            return true;
        }

        if report[0] == report[1] {
            return false;
        }

        let up = report[0] < report[1];
        let down = report[0] > report[1];

        for i in 0..(report.len() - 1) {
            if (report[i] < report[i + 1]) && !up {
                return false;
            }

            if (report[i] > report[i + 1]) && !down {
                return false;
            }

            if (report[i] - report[i + 1]).abs() < 1 {
                return false;
            }

            if (report[i] - report[i + 1]).abs() > 3 {
                return false;
            }
        }

        true
    }

    let safe = reports
        .iter()
        .map(check)
        .filter(|i| *i)
        .collect::<Vec<bool>>()
        .len();

    safe
}

fn part02(content: &String) -> usize {
    let reports = parse(content);
    fn check(report: &Vec<i32>) -> bool {
        if report.len() <= 1 {
            return true;
        }

        if report[0] == report[1] {
            return false;
        }

        let up = report[0] < report[1];
        let down = report[0] > report[1];

        for i in 0..(report.len() - 1) {
            if (report[i] < report[i + 1]) && !up {
                return false;
            }

            if (report[i] > report[i + 1]) && !down {
                return false;
            }

            if (report[i] - report[i + 1]).abs() < 1 {
                return false;
            }

            if (report[i] - report[i + 1]).abs() > 3 {
                return false;
            }
        }

        true
    }

    fn meta_check(report: &Vec<i32>) -> bool {
        if check(report) {
            return true;
        }

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            if check(&report) {
                return true;
            }
        }

        false
    }

    let safe = reports
        .iter()
        .map(meta_check)
        .filter(|i| *i)
        .collect::<Vec<bool>>()
        .len();

    safe
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = aoc_2024::io::read_input("02")?;

    println!("{}", part01(&content));
    println!("{}", part02(&content));

    Ok(())
}
