// [##...##] (0,3,4,6) (1,2,4,5,6) (0,1,2,5,6) (0,1,3,5) (0,2,3,4,6) {29,26,26,12,9,26,26}
// target:
// 1100011
// sources:
// 1001101
// 0111111
// 1110011
// 1101010
// 1011101

use std::io::{BufRead as _, stdin};

fn main() {
    let mut total = 0;
    for readline in stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let mut target = vec![];
        for c in line.chars().skip(1) {
            match c {
                '#' => target.push(true),
                '.' => target.push(false),
                _ => break,
            }
        }

        let mut sources = vec![];
        let mut last_source = vec![];
        let mut last_num = 0;
        for c in line.chars() {
            match c {
                '(' => {
                    last_source = vec![];
                }
                '0'..='9' => {
                    last_num = last_num * 10 + (c as usize - '0' as usize);
                }
                ',' => {
                    while last_source.len() < last_num {
                        last_source.push(false);
                    }
                    last_source.push(true);
                    last_num = 0;
                }
                ')' => {
                    while last_source.len() < last_num {
                        last_source.push(false);
                    }
                    last_source.push(true);
                    while last_source.len() < target.len() {
                        last_source.push(false);
                    }
                    sources.push(last_source);
                    last_source = vec![];
                    last_num = 0;
                }
                '{' => break,
                _ => {}
            }
        }

        let n = target.len();
        let m = sources.len();

        let mut answer = m;
        for mask in 1..(1 << m) {
            let mut result = vec![false; n];
            for i in 0..n {
                let mut cnt = 0;
                for (j, source) in sources.iter().enumerate() {
                    if mask & (1 << j) == 0 {
                        continue;
                    }

                    if source[i] {
                        cnt += 1;
                    }
                }
                result[i] = cnt % 2 == 1;
            }

            let mut same = true;
            for i in 0..n {
                same &= result[i] == target[i];
            }

            if same {
                let mut cnt = 0;
                for i in 0..m {
                    if (mask & (1 << i)) != 0 {
                        cnt += 1;
                    }
                }
                answer = answer.min(cnt);
            }
        }
        total += answer;

        println!("answer: {}", answer);
    }

    println!("total: {}", total);
}
