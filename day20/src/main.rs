use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn label(grid: &[&[u8]], x: usize, y: usize) -> Option<String> {
    if x < 2 || y < 2 || x >= grid[0].len() - 2 || y >= grid.len() - 2 {
        return None;
    }
    for d in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let next_1: u8 = grid[(y as isize + d.1) as usize][(x as isize + d.0) as usize];
        let next_2: u8 = grid[(y as isize + 2 * d.1) as usize][(x as isize + 2 * d.0) as usize];
        if next_1 <= b'Z' && next_1 >= b'A' {
            if d.0 + d.1 < 0 {
                return Some(format!("{}{}", next_2 as char, next_1 as char));
            } else {
                return Some(format!("{}{}", next_1 as char, next_2 as char));
            }
        }
    }
    None
}

fn run(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut labels = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'.' {
                if let Some(label) = label(&grid, x, y) {
                    labels.entry(label).or_insert(vec![]).push((x, y));
                }
            }
        }
    }
    let entry: (usize, usize) = labels["AA"][0];
    let path = dijkstra(
        &entry,
        |&state| {
            let mut succs = vec![];
            for d in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = (state.0 as isize + d.0) as usize;
                let y = (state.1 as isize + d.1) as usize;
                let next: u8 = grid[y][x];
                if next == b'.' {
                    succs.push(((x, y), 1));
                }
                if next >= b'A' && next <= b'Z' {
                    if let Some(label) = label(&grid, state.0, state.1) {
                        let pair = &labels[&label];
                        if pair.len() == 2 {
                            let other = if pair[0] == state { pair[1] } else { pair[0] };
                            succs.push((other, 1));
                        }
                    }
                }
            }
            succs.into_iter()
        },
        |&state| labels["ZZ"][0] == state,
    );
    let p1 = path.unwrap().1;

    let entry: (usize, usize, usize) = (0, entry.0, entry.1);
    let path = dijkstra(
        &entry,
        |&state| {
            let mut succs = vec![];
            for d in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = (state.1 as isize + d.0) as usize;
                let y = (state.2 as isize + d.1) as usize;
                let next: u8 = grid[y][x];
                if next == b'.' {
                    succs.push(((state.0, x, y), 1));
                }
                if next >= b'A' && next <= b'Z' {
                    if let Some(label) = label(&grid, state.1, state.2) {
                        let pair = &labels[&label];
                        if pair.len() == 2 {
                            let other = if pair[0] == (state.1, state.2) {
                                pair[1]
                            } else {
                                pair[0]
                            };
                            let down = x > 2 && x < grid[0].len() - 3 && y > 2 && y < grid.len() - 3;
                            if state.0 > 0 || down {
                                let depth = if down { state.0 + 1 } else { state.0 - 1 };
                                succs.push(((depth, other.0, other.1), 1));
                            }
                        }
                    }
                }
            }
            succs.into_iter()
        },
        |&state| state.0 == 0 && labels["ZZ"][0] == (state.1, state.2),
    );
    let p2 = path.unwrap().1;
    (p1, p2)
}
