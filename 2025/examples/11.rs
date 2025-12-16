use std::{
    collections::{BTreeMap, btree_map::Entry},
    io::{self, BufRead as _},
};

fn input() -> BTreeMap<String, Vec<String>> {
    let mut adj_list: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for readline in io::stdin().lock().lines() {
        let Ok(line) = readline else {
            break;
        };

        let Some(id) = line.find(':') else {
            panic!("input format wrong! Couldn't find ':' in {}", line);
        };

        let from = line.get(0..id).unwrap();
        for to in line
            .get(4..)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()
        {
            let adj = adj_list.entry(to.to_string());
            match adj {
                Entry::Vacant(vacant) => {
                    vacant.insert(vec![from.to_string()]);
                }
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().push(from.to_string());
                }
            }
        }
    }

    adj_list
}

struct Solver {
    adj_list: BTreeMap<String, Vec<String>>,
    count: BTreeMap<String, i64>,
}

impl Solver {
    pub fn new(adj_list: BTreeMap<String, Vec<String>>) -> Solver {
        Solver {
            adj_list,
            count: BTreeMap::new(),
        }
    }

    pub fn solve(&mut self, source: String, target: String) -> i64 {
        self.count = BTreeMap::new();
        self.count.insert(source, 1);
        self.recursive(target)
    }

    fn recursive(&mut self, node: String) -> i64 {
        let Some(list) = self.adj_list.get(&node) else {
            return 0;
        };
        let list = list.clone();

        let mut cur_count = 0;
        for adj in list.iter() {
            if !self.count.contains_key(adj) {
                cur_count += self.recursive(adj.to_string());
            } else {
                cur_count += self.count.get(adj).unwrap();
            }
        }

        self.count.insert(node, cur_count);
        cur_count
    }
}

fn main() {
    let adj_list = input();

    let mut solver = Solver::new(adj_list.clone());
    println!(
        "part one: {}",
        solver.solve(String::from("you"), String::from("out"))
    );

    let svr_fft = solver.solve(String::from("svr"), String::from("fft"));
    let fft_dac = solver.solve(String::from("fft"), String::from("dac"));
    let dac_out = solver.solve(String::from("dac"), String::from("out"));
    println!("part two: {}", svr_fft * fft_dac * dac_out);
}
