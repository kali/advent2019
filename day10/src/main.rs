use num::Integer;
use std::collections::HashMap;
use std::fs;

fn field(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect()
}

fn los(station: (isize, isize), ast: (isize, isize)) -> ((isize, isize), usize) {
    let dx = ast.0 - station.0;
    let dy = ast.1 - station.1;
    let gcd = dx.abs().gcd(&dy.abs());
    let dist = dx * dx + dy * dy;
    ((dx / gcd, dy / gcd), dist as usize)
}

fn polar(
    field: &[(isize, isize)],
    station: (isize, isize),
) -> HashMap<(isize, isize), Vec<((isize, isize), usize)>> {
    let mut map = HashMap::new();
    field.iter().filter(|&a| *a != station).for_each(|a| {
        let los = los(station, *a);
        map.entry(los.0).or_insert(vec![]).push((*a, los.1));
    });
    map
}

fn part_1(field: &[(isize, isize)]) -> (usize, (isize, isize)) {
    field
        .iter()
        .map(|station| (polar(field, *station).len(), *station))
        .max()
        .unwrap()
}

fn vaporized(field: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let station = part_1(field).1;
    let polar = polar(&field, station);
    let mut asteroids: Vec<_> = field
        .iter()
        .copied()
        .filter(|&a| a != station)
        .map(|a| {
            let (ori, dist) = los(station, a);
            let rotation = polar[&ori].iter().filter(|a| a.1 < dist).count();
            let angle = (ori.0 as f64)
                .atan2(-ori.1 as f64)
                .rem_euclid(2.0 * std::f64::consts::PI);
            (rotation, (angle * 10000000.0) as i64, a)
        })
        .collect();
    asteroids.sort();
    asteroids.iter().map(|a| a.2).collect()
}

fn main() {
    let field = field(&fs::read_to_string("input").unwrap());
    dbg!(part_1(&field));
    dbg!(vaporized(&field)[199]);
}

#[test]
fn test_smallish() {
    let input = r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"#;
    let field = field(input);
    assert_eq!(part_1(&field).1, (8, 3));
    let vaporized = vaporized(&field);
    println!("{:?}", vaporized);
    assert_eq!(vaporized[4], (9, 2));
    assert_eq!(vaporized[5], (11, 1));
    assert_eq!(vaporized[6], (12, 1));
    assert_eq!(vaporized[7], (11, 2));
    assert_eq!(vaporized[8], (15, 1));
    assert_eq!(vaporized[9], (12, 2));
    assert_eq!(vaporized[10], (13, 2));
    assert_eq!(vaporized[11], (14, 2));
    assert_eq!(vaporized[12], (15, 2));
}

#[test]
fn test_medium() {
    let input = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;
    let field = field(input);
    assert_eq!(part_1(&field), (210, (11, 13)));
    let vaporized = vaporized(&field);
    assert_eq!(vaporized[0], (11, 12));
    assert_eq!(vaporized[9], (12, 8));
}
