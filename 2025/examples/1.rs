use std::io::{self, BufRead as _};

fn input() -> Vec<String> {
    let mut input = vec![];
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        input.push(line);
    }

    input
}

fn part_one(ops: &Vec<String>) -> i32 {
    let mut password = 0;
    let mut x = 50;
    for op in ops.iter() {
        let shift = op.get(1..).unwrap().parse::<i32>().unwrap();
        match op.chars().next().unwrap() {
            'L' => x = ((x - shift) % 100 + 100) % 100,
            'R' => x = (x + shift) % 100,
            _ => panic!("ops invalid: {}", op),
        }

        if x == 0 {
            password += 1;
        }
    }

    password
}

fn part_two(ops: &Vec<String>) -> i32 {
    let mut password = 0;
    let mut x = 50;
    for op in ops.iter() {
        let mut shift = op.get(1..).unwrap().parse::<i32>().unwrap();
        password += shift / 100;
        shift %= 100;
        match op.chars().next().unwrap() {
            'L' => {
                if x > 0 && x - shift <= 0 {
                    password += 1;
                }
                x = (x - shift + 100) % 100;
            }
            'R' => {
                if shift != 0 && (x + shift) % 100 < x {
                    password += 1;
                }
                x = (x + shift) % 100;
            }
            _ => panic!("ops invalid: {}", op),
        };
    }

    password
}

fn main() {
    let ops = input();

    println!("part one: {}", part_one(&ops));
    println!("part two: {}", part_two(&ops));
}
