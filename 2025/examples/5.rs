use std::io::{self, BufRead as _};

fn input() -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut freshes = vec![];
    let mut avail = vec![];

    let mut start_avail = false;
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        if line.is_empty() {
            start_avail = true;
            continue;
        }

        if !start_avail {
            let tmp: Vec<i64> = line.split("-").map(|x| x.parse::<i64>().unwrap()).collect();
            freshes.push((tmp[0], tmp[1]));
        } else {
            avail.push(line.parse::<i64>().unwrap());
        }
    }

    (freshes, avail)
}

fn main() {
    let (mut freshes, availables) = input();
    freshes.sort();

    let mut part_one = 0;
    for &avail in availables.iter() {
        let mut is_fresh = false;
        for fresh in freshes.iter().skip_while(|f| avail < f.0) {
            is_fresh |= fresh.0 <= avail && avail <= fresh.1;
        }
        if is_fresh {
            part_one += 1;
        }
    }

    println!("part one: {}", part_one);

    let mut part_two = 0;
    let mut l = 0;
    let mut r = 0;
    let mut r_num = 0;
    while l < freshes.len() {
        if r == l {
            r_num = freshes[r].1;
            r = l + 1;
        }
        while r < freshes.len() && freshes[r].0 <= r_num {
            r_num = r_num.max(freshes[r].1);
            r += 1;
        }

        part_two += r_num - freshes[l].0 + 1;
        l = r;
    }

    println!("part two: {}", part_two);
}
