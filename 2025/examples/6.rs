use std::io::{self, BufRead};

fn input() -> (Vec<Vec<i64>>, Vec<char>) {
    let mut nums = vec![];
    let mut ops = vec![];
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        if let Some(char) = line.chars().next() {
            if char == '*' || char == '+' {
                ops = line
                    .split_whitespace()
                    .map(|c| c.chars().next().unwrap())
                    .collect();
                break;
            } else {
                nums.push(
                    line.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect(),
                );
            }
        }
    }

    (nums, ops)
}

fn main() {
    let (nums, ops) = input();

    let mut total = 0;
    for col in 0..nums[0].len() {
        let mut sum = nums[0][col];
        for row in 1..nums.len() {
            match ops[col] {
                '*' => sum *= nums[row][col],
                '+' => sum += nums[row][col],
                _ => {}
            }
        }
        total += sum;
    }

    println!("total: {}", total);
}
