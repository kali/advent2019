use std::collections::HashMap;
use num::Integer;

type State = [([i16; 3], [i16; 3]); 4];

fn parse(input: &str) -> State {
    let mut state:State = unsafe { std::mem::zeroed() };
    for (ix, b) in input.trim().lines().enumerate() {
        let c = b
            .trim()
            .chars()
            .filter(|c| !"<>xyz= ".contains(|c2| c2 == *c))
            .collect::<String>();
        let mut c = c.split(",").map(|n| n.parse::<i16>().unwrap());
        state[ix].0 = [c.next().unwrap(), c.next().unwrap(), c.next().unwrap()];
    }
    state
}

fn step(state: &mut State, coord_filter: Option<usize>) {
    for a in 0..state.len() {
        for b in 0..state.len() {
            for c in 0..3 {
                if coord_filter.is_none() || coord_filter == Some(c) {
                    state[a].1[c] += (state[b].0[c] - state[a].0[c]).signum();
                }
            }
        }
    }
    for a in 0..state.len() {
        for c in 0..3 {
            if coord_filter.is_none() || coord_filter == Some(c) {
                state[a].0[c] += state[a].1[c];
            }
        }
    }
}

fn part_1(input: &str) -> i16 {
    let mut state = parse(input);
    for _step in 0..1000 {
        step(&mut state, None);
    }
    state.iter()
        .map(|s| {
            s.0.iter().map(|c| c.abs()).sum::<i16>()
                * s.1.iter().map(|c| c.abs()).sum::<i16>()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let orbits = (0usize..3).map(|c| {
        let mut seen = HashMap::new();
        let mut state = parse(input);
        for i in 0usize.. {
            step(&mut state, Some(c));
            if let Some(j) = seen.get(&state).copied() {
                return (i-j, j)
            }
            seen.insert(state, i);
        }
        unreachable!();
    }).collect::<Vec<_>>();
    let orbit = orbits.iter().fold(1, |acc, o| acc.lcm(&o.0));
    let delay = orbits.iter().map(|p| p.1).max().unwrap() as usize;
    return delay + orbit;
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}
