use std::io::{self, BufRead as _};

#[derive(Debug, Clone)]
struct Shape {
    grid: Vec<Vec<bool>>,
}

impl Shape {
    pub fn new() -> Shape {
        Shape { grid: vec![] }
    }
}

#[derive(Debug, Clone)]
struct Case {
    n: usize,
    m: usize,
    qty: [usize; 6],
}

struct Solver {
    n: usize,
    m: usize,
    shape: Shape,
    qty: usize,
    // [x][y][2^6][qty]
    dp: Vec<Vec<Vec<Vec<bool>>>>,
}

impl Solver {
    pub fn solve(shapes: Vec<Shape>, case: Case) -> bool {
        let dp = vec![vec![vec![vec![false; 100]; 1 << 9]; case.m]; case.n];

        let mut solver = Solver {
            n: case.n,
            m: case.m,
            shape: Shape::new(),
            qty: 0,
            dp,
        };
        solver.start();
        for i in 0..6 {
            solver.shape = shapes[i].clone();
            solver.qty = case.qty[i];

            solver.compute();
            if i == 4 {
                for qty in 0..=2 {
                    println!("qty: {}", qty);
                    for x in 0..solver.n {
                        for y in 0..solver.m {
                            print!(
                                "{}",
                                solver.dp[x][y].iter().fold(false, |acc, d| acc | d[qty]) as u8
                            );
                        }
                        println!();
                    }
                    println!("{}", solver.dp[0][4][0][2]);
                    println!();
                }
                for mask in 0..(1 << 9) {
                    let target = solver.dp[0][1][mask][2];
                    if target == true {
                        println!("mask: {}", mask);
                    }
                }
            }
            solver.transition();
        }

        solver.valid()
    }

    fn start(&mut self) {
        for i in 0..self.n {
            for j in 0..self.m {
                self.dp[i][j][0][0] = true;
            }
        }
    }

    fn transition(&mut self) {
        let mut dp = vec![vec![vec![vec![false; 100]; 1 << 9]; self.m]; self.n];
        for i in 0..self.n {
            for j in 0..self.m {
                for mask in 0..(1 << 9) {
                    dp[i][j][mask][0] = self.dp[i][j][mask][self.qty];
                }
            }
        }

        self.dp = dp;
    }

    fn get_shape(&self, rotation: usize) -> Shape {
        let mut shape = self.shape.clone();

        for _ in 0..rotation {
            let mut new_shape = shape.clone();
            for i in 0..3 {
                for j in 0..3 {
                    new_shape.grid[i][j] = shape.grid[2 - j][i];
                }
            }
            shape = new_shape;
        }

        shape
    }

    fn get_mask(shape: &Shape) -> usize {
        let mut mask = 0;
        for i in 0..3 {
            for j in 0..3 {
                mask |= (shape.grid[i][j] as usize) << ((3 * (3 - i)) - j - 1);
            }
        }

        mask
    }

    fn shift_mask(mask: usize, x: usize, y: usize) -> usize {
        let mut new_mask = 0;
        for i in 0..3 {
            let a = i + x;
            if a >= 3 {
                break;
            }
            for j in 0..3 {
                let b = j + y;
                if b >= 3 {
                    break;
                }
                if mask & (1 << ((3 * (3 - a)) - b - 1)) != 0 {
                    new_mask |= 1 << ((3 * (3 - i)) - j - 1);
                }
            }
        }

        new_mask
    }

    fn compute(&mut self) {
        for qty in 0..=self.qty {
            for i in 0..self.n - 3 {
                for j in 0..self.m - 3 {
                    for mask in 0..(1 << 9) {
                        for x in 0..2 {
                            for y in 0..2 {
                                self.dp[i + x][j + y][Self::shift_mask(mask, x, y)][qty] |=
                                    self.dp[i][j][mask][qty];
                            }
                        }
                    }
                }
            }
            if qty == self.qty {
                break;
            }
            for i in 0..self.n - 2 {
                for j in 0..self.m - 2 {
                    for mask in 0..(1 << 9) {
                        if !self.dp[i][j][mask][qty] {
                            continue;
                        }
                        for rotation in 0..4 {
                            let shape = self.get_shape(rotation);

                            if (mask & Self::get_mask(&shape)) != 0 {
                                continue;
                            }

                            self.dp[i][j][mask | Self::get_mask(&shape)][qty + 1] = true;
                        }
                    }
                }
            }
        }
    }

    fn valid(&self) -> bool {
        for i in 0..self.n - 2 {
            for j in 0..self.m - 2 {
                for mask in 0..(1 << 9) {
                    if self.dp[i][j][mask][0] {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn input_shapes() -> Vec<Shape> {
    let mut shapes = vec![];
    for _ in 0..6 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut shape = Shape::new();
        for _ in 0..3 {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            shape.grid.push(
                line.trim()
                    .chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect(),
            );
        }
        shapes.push(shape);
        io::stdin().read_line(&mut line).unwrap();
    }

    shapes
}

fn input_cases() -> Vec<Case> {
    let mut cases = vec![];
    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let splitted_line = line.split(':').collect::<Vec<&str>>();
        let splitted_size = splitted_line[0].split('x').collect::<Vec<&str>>();
        let splitted_qty = splitted_line[1]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        cases.push(Case {
            n: splitted_size[0].parse::<usize>().unwrap(),
            m: splitted_size[1].parse::<usize>().unwrap(),
            qty: splitted_qty
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }

    cases
}

fn main() {
    let shapes = input_shapes();
    let cases = input_cases();

    let mut answer = 0;
    for case in cases.iter() {
        let valid = Solver::solve(shapes.clone(), case.clone());
        if valid {
            answer += 1;
        }
        println!("{} {:?}", valid, case);
    }

    // wrong answer, this DP won't work
    println!("{}", answer);
}
