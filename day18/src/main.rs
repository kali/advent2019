use std::collections::HashMap;
use std::collections::VecDeque;

use pathfinding::directed::astar::astar;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::directed::fringe::fringe;

fn new_keys_from(grid: &[Vec<u8>], x: u8, y: u8, keys: u32) -> Vec<(u8, u8, u8, usize)> {
    // x, y, key, cost
    let mut found = HashMap::<(u8, u8), usize>::new();
    let mut todo = VecDeque::new();
    todo.push_back((x, y));
    found.insert((x, y), 0);
    let mut new_keys = Vec::new();
    while let Some((x, y)) = todo.pop_front() {
        let cost = found[&(x, y)];
        for d in &[(1i8, 0i8), (-1, 0), (0, -1), (0, 1)] {
            let x = (x as i8 + d.0) as u8;
            let y = (y as i8 + d.1) as u8;
            if found.contains_key(&(x, y)) {
                continue;
            } else {
                found.insert((x, y), cost + 1);
            }
            let blk = grid[y as usize][x as usize];
            if blk >= b'a' && blk <= b'z' && (keys & (1 << (blk - b'a')) == 0) {
                new_keys.push((x, y, blk - b'a', cost + 1));
                continue;
            }
            if blk == b'@'
                || blk == b'.'
                || (blk >= b'a' && blk <= b'z')
                || (blk >= b'A' && blk <= b'Z' && (keys & (1 << (blk - b'A')) != 0))
            {
                todo.push_back((x, y));
            }
        }
    }
    new_keys
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    bots: Vec<(u8, u8)>,
    keys: u32,
}

fn run(grid: &[Vec<u8>], start: State) -> usize {
    let key_count = grid
        .iter()
        .map(|lines| lines.iter().filter(|&&c| c >= b'a' && c <= b'z').count())
        .sum::<usize>();
    let all_keys = (1 << key_count) - 1;

    let path = astar(
        &start,
        |state| {
            let state = state.clone();
            state
                .bots
                .clone()
                .into_iter()
                .enumerate()
                .flat_map(move |(ix, bot)| {
                    let state = state.clone();
                    new_keys_from(&grid, bot.0, bot.1, state.keys)
                        .into_iter()
                        .map(move |(x, y, k, c)| {
                            let mut state = state.clone();
                            state.bots[ix].0 = x;
                            state.bots[ix].1 = y;
                            state.keys |= 1 << k;
                            (state, c)
                        })
                })
        },
        |state| (all_keys - state.keys).count_ones() as usize,
        |state| state.keys == all_keys,
    );
    path.unwrap().1
}

fn grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut grid: Vec<Vec<u8>> = grid(&input);

    let start = grid
        .iter()
        .enumerate()
        .filter_map(|(y, l)| {
            l.iter()
                .position(|&b| b == b'@')
                .map(|x| (x as u8, y as u8))
        })
        .next()
        .unwrap();

    let state = State {
        bots: vec![start],
        keys: 0,
    };
    dbg!(run(&grid, state));

    grid[start.0 as usize][start.1 as usize - 1] = b'#';
    grid[start.0 as usize][start.1 as usize + 1] = b'#';
    grid[start.0 as usize - 1][start.1 as usize] = b'#';
    grid[start.0 as usize + 1][start.1 as usize] = b'#';

    let state = State {
        bots: vec![
            (start.0 - 1, start.1 - 1),
            (start.0 - 1, start.1 + 1),
            (start.0 + 1, start.1 - 1),
            (start.0 + 1, start.1 + 1),
        ],
        keys: 0,
    };
    dbg!(run(&grid, state));
}

#[test]
fn test() {
    let grid = grid(
        r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#,
    );
    assert_eq!(new_keys_from(&grid, 6, 3, 0).len(), 2);
    assert_eq!(new_keys_from(&grid, 18, 3, 0b11111).len(), 1);
    let s = State {
        keys: 0,
        bots: vec![(6, 3)],
    };
    let l = run(&grid, s);
    dbg!(l);
    panic!();
}
