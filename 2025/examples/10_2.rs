use std::{
    i32,
    io::{BufRead as _, stdin},
};

use microlp::Problem;

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let mut target = vec![];
    let mut last_num = 0;
    for c in input.chars().skip_while(|c| *c != '{').skip(1) {
        match c {
            '0'..='9' => last_num = last_num * 10 + (c as i32 - '0' as i32),
            ',' | '}' => {
                target.push(last_num);
                last_num = 0;
            }
            _ => break,
        }
    }

    let mut sources = vec![];
    let mut last_source = vec![];
    let mut last_num = 0;
    for c in input.chars() {
        match c {
            '(' => {
                last_source = vec![];
            }
            '0'..='9' => {
                last_num = last_num * 10 + (c as usize - '0' as usize);
            }
            ',' => {
                last_source.push(last_num);
                last_num = 0;
            }
            ')' => {
                last_source.push(last_num);
                sources.push(last_source);
                last_source = vec![];
                last_num = 0;
            }
            '{' => break,
            _ => {}
        }
    }

    let n = target.len();
    let m = sources.len() + 1;
    let mut result = vec![vec![0; m]; n];
    for (i, tar) in target.iter().enumerate() {
        result[i][m - 1] = *tar;
    }
    for (i, source) in sources.iter().enumerate() {
        for (_, src) in source.iter().enumerate() {
            result[*src][i] = 1;
        }
    }

    result
}

fn main() {
    let mut total = 0;
    for readline in stdin().lock().lines() {
        let Ok(input) = readline else {
            break;
        };

        let matrix = parse_input(input);
        println!("{:?}", matrix);

        let n = matrix.len();
        let m = matrix[0].len() - 1;
        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);
        let mut x = vec![];
        for _ in 0..m {
            x.push(problem.add_integer_var(1.0, (0, i32::MAX)));
        }
        for i in 0..n {
            let mut cons = vec![];
            for j in 0..m {
                if matrix[i][j] == 0 {
                    continue;
                }
                cons.push((x[j], 1.0));
            }
            problem.add_constraint(&cons, microlp::ComparisonOp::Eq, matrix[i][m] as f64);
        }

        let solution = problem.solve().unwrap();
        println!("{:?}\n", solution);

        total += solution.objective().round() as i32;
    }

    println!("{}", total);
}
