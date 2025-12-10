use std::{
    collections::{BTreeMap, VecDeque},
    io::{BufRead, stdin},
};

fn main() {
    let mut points: Vec<(i32, i32)> = vec![];

    for readline in stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let point: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        points.push((point[0], point[1]));
    }

    let mut compressed_x: BTreeMap<i32, i32> = BTreeMap::new();
    let mut compressed_y: BTreeMap<i32, i32> = BTreeMap::new();
    for i in points.iter() {
        compressed_x.insert(i.0, 0);
        compressed_y.insert(i.1, 0);
    }
    let mut n = 1_usize;
    for x in compressed_x.iter_mut() {
        *x.1 = n as i32;
        n += 1;
    }
    let mut m = 1_usize;
    for y in compressed_y.iter_mut() {
        *y.1 = m as i32;
        m += 1;
    }
    n += 1;
    m += 1;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..points.len() - 1 {
        let p = (compressed_x[&points[i].0], compressed_y[&points[i].1]);
        let q = (
            compressed_x[&points[i + 1].0],
            compressed_y[&points[i + 1].1],
        );

        if p.0 == q.0 {
            for ver in p.1.min(q.1)..=p.1.max(q.1) {
                grid[p.0 as usize][ver as usize] = 1;
            }
        } else {
            for hor in p.0.min(q.0)..=p.0.max(q.0) {
                grid[hor as usize][p.1 as usize] = 1;
            }
        }
    }

    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((0, 0));
    grid[0][0] = -1;
    while !q.is_empty() {
        let Some(a) = q.pop_front() else {
            break;
        };

        for d in dir.iter() {
            let nxt = (a.0 + d.0, a.1 + d.1);
            if nxt.0 < 0 || nxt.0 >= n as i32 || nxt.1 < 0 || nxt.1 >= m as i32 {
                continue;
            }

            if grid[nxt.0 as usize][nxt.1 as usize] == 0 {
                grid[nxt.0 as usize][nxt.1 as usize] = -1;
                q.push_back(nxt);
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                grid[i][j] = 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == -1 {
                grid[i][j] = 0;
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if i > 0 {
                grid[i][j] += grid[i - 1][j];
            }
            if j > 0 {
                grid[i][j] += grid[i][j - 1];
            }
            if i > 0 && j > 0 {
                grid[i][j] -= grid[i - 1][j - 1];
            }
        }
    }

    let mut best = 0_i64;
    for p in points.iter() {
        for q in points.iter() {
            let a = (p.0.min(q.0), p.1.min(q.1));
            let b = (p.0.max(q.0), p.1.max(q.1));

            let comp_a = (compressed_x[&a.0], compressed_y[&a.1]);
            let comp_b = (compressed_x[&b.0], compressed_y[&b.1]);
            let mut grid_cnt = grid[comp_b.0 as usize][comp_b.1 as usize];
            grid_cnt -= grid[comp_a.0 as usize - 1][comp_b.1 as usize];
            grid_cnt -= grid[comp_b.0 as usize][comp_a.1 as usize - 1];
            grid_cnt += grid[comp_a.0 as usize - 1][comp_a.1 as usize - 1];

            let mut x = (comp_a.0 - comp_b.0).abs() + 1;
            let mut y = (comp_a.1 - comp_b.1).abs() + 1;
            if x * y == grid_cnt {
                x = (a.0 - b.0).abs() + 1;
                y = (a.1 - b.1).abs() + 1;
                best = best.max(x as i64 * y as i64);
            }
        }
    }

    println!("{}", best);
}
