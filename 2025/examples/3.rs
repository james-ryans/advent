use std::io::{self, BufRead as _};

fn main() {
    let mut part_one = 0;
    let mut part_two = 0_i64;
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let mut num = 0;
        for (i, a) in line.chars().enumerate() {
            for b in line.chars().skip(i + 1) {
                let tmp = (a as i32 - '0' as i32) * 10 + (b as i32 - '0' as i32);
                num = num.max(tmp);
            }
        }
        part_one += num;

        let mut num = String::new();
        let mut last = 0;
        for i in 0..12 {
            let mut best = '0';
            for (j, x) in line.chars().enumerate().skip(last) {
                if j + 12 - i - 1 == line.len() {
                    break;
                }
                if x > best {
                    best = x;
                    last = j + 1;
                }
            }
            num.push(best);
        }
        part_two += num.parse::<i64>().unwrap();
    }

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}
