use std::io;

fn input() -> Vec<(i64, i64)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .split(',')
        .map(|a| {
            let x: Vec<&str> = a.split('-').collect();
            (x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap())
        })
        .collect()
}

fn repeated(num: String) -> bool {
    let n = num.len();

    for div in 1..n {
        if n % div != 0 {
            continue;
        }

        let substr = num.get(0..div).unwrap();
        if substr.repeat(n / div) == num {
            return true;
        }
    }

    false
}

fn main() {
    let ranges = input();

    let mut part_one = 0;
    let mut part_two = 0;
    for range in ranges.iter() {
        for num in range.0..=range.1 {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0
                && num_str.get(0..num_str.len() / 2) == num_str.get(num_str.len() / 2..)
            {
                part_one += num;
            }

            if repeated(num_str) {
                part_two += num;
            }
        }
    }

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}
