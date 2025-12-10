use std::io::{BufRead, stdin};

fn main() {
    let mut points: Vec<(i32, i32)> = vec![];

    for readline in stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let point: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        points.push((point[0], point[1]));
    }

    let mut best = 0_i64;
    for p in points.iter() {
        for q in points.iter() {
            let x = (p.0 - q.0).abs() + 1;
            let y = (p.1 - q.1).abs() + 1;
            best = best.max(x as i64 * y as i64);
        }
    }

    println!("{}", best);
}
