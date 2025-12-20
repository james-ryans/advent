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

struct Combi {
    len: usize,
    combi: Vec<usize>,
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

// Combi
// 0 + 0 + 2 = 3x7
// 0 + 0 + 3 = 3x7
// 0 + 1 + 3 = 3x7
// 3 + 3 + 5 = 3x7
// 0 + 0 = 3x5
// 0 + 1 = 3x5
// 0 + 2 = 3x5
// 0 + 3 = 3x5
// 1 + 3 = 3x5
// 3 + 4 = 3x5
// 3 + 5 = 3x5
// 4 + 4 = 4x4

fn main() {
    let shapes = input_shapes();
    let cases = input_cases();

    let combies = vec![
        Combi {
            len: 7,
            combi: vec![0, 1, 3],
        },
        Combi {
            len: 7,
            combi: vec![0, 0, 2],
        },
        Combi {
            len: 7,
            combi: vec![3, 3, 5],
        },
        Combi {
            len: 7,
            combi: vec![0, 0, 3],
        },
        Combi {
            len: 5,
            combi: vec![1, 3],
        },
        Combi {
            len: 5,
            combi: vec![3, 4],
        },
        Combi {
            len: 5,
            combi: vec![3, 5],
        },
        Combi {
            len: 5,
            combi: vec![0, 0],
        },
        Combi {
            len: 5,
            combi: vec![0, 1],
        },
        Combi {
            len: 5,
            combi: vec![0, 2],
        },
        Combi {
            len: 5,
            combi: vec![0, 3],
        },
        Combi {
            len: 3,
            combi: vec![0],
        },
        Combi {
            len: 3,
            combi: vec![1],
        },
        Combi {
            len: 3,
            combi: vec![2],
        },
        Combi {
            len: 3,
            combi: vec![3],
        },
        Combi {
            len: 3,
            combi: vec![4],
        },
        Combi {
            len: 3,
            combi: vec![5],
        },
    ];

    let mut answer = 0;
    for case in cases.iter() {
        let mut qty = case.qty.clone();
        let (mut region, length) = if case.n % 3 == 0 {
            (case.n as i8 / 3, case.m)
        } else if case.m % 3 == 0 {
            (case.m as i8 / 3, case.n)
        } else {
            (case.n.min(case.m) as i8 / 3, case.n.max(case.m))
        };
        let mut cur_length = length;
        for Combi { len, combi } in combies.iter() {
            if length < *len {
                continue;
            }
            loop {
                match combi.len() {
                    3 => {
                        if combi[0] == combi[1] && (qty[combi[0]] < 2 || qty[combi[2]] < 1) {
                            break;
                        }
                        if qty[combi[0]] < 1 || qty[combi[1]] < 1 || qty[combi[2]] < 1 {
                            break;
                        }
                    }
                    2 => {
                        if combi[0] == combi[1] && qty[combi[0]] < 2 {
                            break;
                        }
                        if qty[combi[0]] < 1 || qty[combi[1]] < 1 {
                            break;
                        }
                    }
                    1 => {
                        if qty[combi[0]] < 1 {
                            break;
                        }
                    }
                    _ => {}
                }

                if cur_length < *len {
                    region -= 1;
                    cur_length = length;
                }
                cur_length -= *len;
                for c in combi.iter() {
                    qty[*c] -= 1;
                }
            }
        }

        if region >= 0 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
