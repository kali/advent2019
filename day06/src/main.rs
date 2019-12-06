use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn orbiting(input: &str) -> HashMap<&str, &str> {
    input
        .split_whitespace()
        .map(|l| {
            let (c, s) = l.split(")").tuple_windows().next().unwrap();
            (s, c)
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let orbiting = orbiting(input);
    let mut orbits = HashMap::new();
    orbits.insert("COM", 0);
    while orbits.len() < orbiting.len() + 1 {
        for (sat, center) in &orbiting {
            if let Some(co) = orbits.get(center).map(|x| *x) {
                if !orbits.contains_key(sat) {
                    orbits.insert(sat, co + 1);
                }
            }
        }
    }
    orbits.values().sum::<usize>()
}

fn part_2(input: &str) -> usize {
    let orbiting = orbiting(input);
    let you = path_to(&orbiting, "YOU");
    let san = path_to(&orbiting, "SAN");
    let common = you
        .iter()
        .zip(san.iter())
        .position(|(y, s)| y != s)
        .unwrap();
    you.len() + san.len() - 2 * common - 2
}

fn path_to<'a>(orbiting: &HashMap<&'a str, &'a str>, target: &'a str) -> Vec<&'a str> {
    let mut path = vec![target];
    while path.last().unwrap() != &"COM" {
        path.push(orbiting[path.last().unwrap()]);
    }
    path.reverse();
    path
}

#[test]
fn test_1() {
    assert_eq!(
        42,
        part_1(r#"COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L"#)
    );
}

#[test]
fn test_2() {
    assert_eq!(
        4,
        part_2(r#"COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L K)YOU I)SAN"#)
    );
}
