use std::io::{self, BufRead};

fn input() -> (Vec<Vec<i64>>, Vec<char>) {
    let mut num_grid: Vec<Vec<char>> = vec![];
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
                num_grid.push(line.chars().collect());
            }
        }
    }

    let mut nums = vec![];
    let mut last_nums = vec![];
    for y in 0..num_grid[0].len() {
        let mut num = 0;
        for x in 0..num_grid.len() {
            if num_grid[x][y] != ' ' {
                num = num * 10 + num_grid[x][y] as i64 - '0' as i64;
            }
        }
        if num == 0 {
            nums.push(last_nums);
            last_nums = vec![];
        } else {
            last_nums.push(num);
        }
    }
    nums.push(last_nums);

    (nums, ops)
}

fn main() {
    let (nums, ops) = input();
    for row in nums.iter() {
        println!("{:?}", row);
    }

    let mut total = 0;
    for (i, row) in nums.iter().enumerate() {
        let mut sum = *row.first().unwrap();
        for num in row.iter().skip(1) {
            match ops[i] {
                '*' => sum *= num,
                '+' => sum += num,
                _ => {}
            }
        }
        total += sum;
    }

    println!("total: {}", total);
}
