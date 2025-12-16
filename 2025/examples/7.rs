use std::io::{self, BufRead as _};

fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        grid.push(line.chars().collect());
    }

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            match grid[x][y] {
                'S' => {
                    grid[x + 1][y] = '|';
                }
                '^' => {
                    if grid[x - 1][y] == '|' {
                        grid[x][y - 1] = '|';
                        grid[x][y + 1] = '|';
                    }
                }
                '.' => {
                    if x > 0 && grid[x - 1][y] == '|' {
                        grid[x][y] = '|';
                    }
                }
                _ => {}
            }
        }
    }

    let mut split = 0;
    for x in 1..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '^' && grid[x - 1][y] == '|' {
                split += 1;
            }
        }
    }

    println!("part one: {}", split);

    let mut count = vec![vec![0_i64; grid[0].len()]; grid.len()];
    for y in 0..grid[0].len() {
        if grid[0][y] == 'S' {
            count[0][y] = 1;
            break;
        }
    }

    for x in 1..grid.len() {
        for y in 0..grid[x].len() {
            match grid[x][y] {
                '|' => {
                    if grid[x - 1][y] == '|' || grid[x - 1][y] == 'S' {
                        count[x][y] += count[x - 1][y];
                    }
                }
                '^' => {
                    // println!("x,y = {},{}", x, y);
                    count[x][y - 1] += count[x - 1][y];
                    count[x][y + 1] += count[x - 1][y];
                }
                _ => {}
            }
        }
    }

    println!("part two: {}", count.last().unwrap().iter().sum::<i64>());
}
