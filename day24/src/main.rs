use std::collections::HashSet;

fn next(state: u32) -> u32 {
    let mut next = state;
    let at = |x, y| x >= 0 && x <= 4 && y >= 0 && y <= 4 && state as i32 & 1 << (y * 5 + x) != 0;
    for x in 0..5 {
        for y in 0..5 {
            let count = at(x - 1, y) as usize
                + at(x, y - 1) as usize
                + at(x + 1, y) as usize
                + at(x, y + 1) as usize;
            if at(x, y) && count != 1 {
                next ^= 1 << y * 5 + x;
            }
            if !at(x, y) && (count == 1 || count == 2) {
                next |= 1 << y * 5 + x;
            }
        }
    }
    next
}

fn next_nested(state: &[u32]) -> Vec<u32> {
    let mut state = state.to_vec();
    state.insert(0, 0);
    state.insert(0, 0);
    state.push(0);
    state.push(0);
    let mut next = state.clone();
    let mask = |x:usize, y:usize| -> i32 { 1 << (y * 5 + x) };
    let at = |x, y, z| (state[z] as i32 & mask(x,y) != 0) as usize;
    for z in 1..state.len() - 1 {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }
                let mut count = 0;
                count += match x {
                    0 => at(1, 2, z + 1) + at(1, y, z),
                    4 => at(3, 2, z + 1) + at(3, y, z),
                    _ => at(x - 1, y, z) + at(x + 1, y, z),
                };
                match y {
                    0 => count += at(2, 1, z + 1) + at(x, 1, z),
                    4 => count += at(2, 3, z + 1) + at(x, 3, z),
                    _ => count += at(x, y - 1, z) + at(x, y + 1, z),
                }
                if x == 2 && y == 1 {
                    count += (0..5).map(|x| at(x, 0, z - 1)).sum::<usize>();
                }
                if x == 2 && y == 3 {
                    count += (0..5).map(|x| at(x, 4, z - 1)).sum::<usize>();
                }
                if x == 1 && y == 2 {
                    count += (0..5).map(|y| at(0, y, z - 1)).sum::<usize>();
                }
                if x == 3 && y == 2 {
                    count += (0..5).map(|y| at(4, y, z - 1)).sum::<usize>();
                }
                if at(x, y, z) == 1 && count != 1 {
                    next[z] ^= 1 << y * 5 + x;
                }
                if at(x, y, z) == 0 && (count == 1 || count == 2) {
                    next[z] |= 1 << y * 5 + x;
                }
            }
        }
    }
    while next[0] == 0 {
        next.remove(0);
    }
    while next.last() == Some(&0) {
        next.pop();
    }
    next
}

fn parse(s: &str) -> u32 {
    s.trim()
        .lines()
        .map(|l| l.bytes().rev().fold(0, |s, c| s << 1 | (c == b'#') as u32))
        .rev()
        .fold(0, |s, l| s << 5 | l)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut state = parse(&input);
    let mut seen = HashSet::new();
    seen.insert(state);
    loop {
        state = next(state);
        if seen.contains(&state) {
            dbg!(state);
            break;
        }
        seen.insert(state);
    }

    let mut state = vec!(parse(&input));
    for _ in 0..200 {
        state = next_nested(&state);
    }
    dbg!(state.iter().map(|s| s.count_ones()).sum::<u32>());
}

#[allow(dead_code)]
fn dump_nested(s: &[u32]) {
    for (depth, l) in s.iter().enumerate() {
        println!("depth: {}", depth);
        let bits = format!("{:025b}", l).chars().rev().collect::<String>();
        for y in 0..5 {
            println!(
                "{}",
                &bits[5 * y..][..5].replace("1", "#").replace("0", ".")
            );
        }
        println!("");
    }
}

#[test]
fn t1() {
    let s0 = parse(
        r#"....#
#..#.
#..##
..#..
#...."#,
    );
    let s1 = parse(
        r#"#..#.
####.
###.#
##.##
.##.."#,
    );
    assert_eq!(next(s0), s1);
    let s2 = parse(
        r#"#####
....#
....#
...#.
#.###"#,
    );
    assert_eq!(next(s1), s2);
}

#[test]
fn t2() {
    let mut s = vec![parse(
        r#"....#
#..#.
#..##
..#..
#...."#,
    )];
    for _ in 0..10 {
        s = next_nested(&s);
    }
    dbg!(s.len());
    assert!(s.iter().all(|l| l & 1 << 12 == 0));
    assert_eq!(s.iter().map(|s| s.count_ones()).sum::<u32>(), 99);
}
