use std::io::{self, BufRead as _};

fn input() -> Vec<Vec<char>> {
    let mut grid = vec![];
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        grid.push(line.chars().collect());
    }

    grid
}

fn remove(grid: &mut Vec<Vec<char>>) -> i32 {
    let adj: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let mut remove_pos = vec![];
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    for x in 0..n {
        for y in 0..m {
            if grid[x as usize][y as usize] == '.' {
                continue;
            }

            let mut papers = 0;
            for d in adj.iter() {
                let dx = x + d.0;
                let dy = y + d.1;
                if 0 <= dx && dx < n && 0 <= dy && dy < m {
                    papers += (grid[dx as usize][dy as usize] == '@') as i32;
                }
            }
            if papers < 4 {
                remove_pos.push((x, y));
            }
        }
    }
    for (x, y) in remove_pos.iter() {
        grid[*x as usize][*y as usize] = '.';
    }

    remove_pos.len() as i32
}

fn main() {
    let mut grid = input();

    let count = remove(&mut grid);
    println!("part one: {}", count);

    let mut total_removed = count;
    loop {
        let remove_count = remove(&mut grid);
        if remove_count == 0 {
            break;
        }

        total_removed += remove_count;
    }

    println!("part two: {}", total_removed);
}
