use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (usize, usize) {
    let mut wires = input.lines().map(|s| {
        let mut pos = (0isize, 0isize); // row, col
        let mut segments = vec![];
        for mov in s.split(",") {
            let dir = mov.as_bytes()[0];
            let len = mov
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<isize>()
                .unwrap();
            let next = match dir {
                b'R' => (pos.0, pos.1 + len),
                b'L' => (pos.0, pos.1 - len),
                b'U' => (pos.0 - len, pos.1),
                b'D' => (pos.0 + len, pos.1),
                _ => panic!(),
            };
            segments.push((pos, next));
            pos = next;
        }
        segments
    });
    let wire_a = wires.next().unwrap();
    let wire_b = wires.next().unwrap();
    let mut inters = vec![];
    for a in &wire_a {
        for b in &wire_b {
            for (h, v) in &[(a, b), (b, a)] {
                if (h.0).0 == (h.1).0 && (v.0).1 == (v.1).1 {
                    let row = (h.0).0;
                    let col = (v.0).1;
                    if row > ((v.0).0).min((v.1).0)
                        && row < ((v.0).0).max((v.1).0)
                        && col > ((h.0).1).min((h.1).1)
                        && col < ((h.0).1).max((h.1).1)
                    {
                        inters.push((row, col));
                    }
                }
            }
        }
    }
    let manhattan = inters
        .iter()
        .map(|pos| pos.0.abs() + pos.1.abs())
        .min()
        .unwrap() as usize;
    let delay = inters
        .iter()
        .map(|pos| {
            let delays = [&wire_a, &wire_b]
                .iter()
                .map(|wire| {
                    let seg_ix = wire
                        .iter()
                        .position(|s| {
                            pos.0 <= (s.0).0.max((s.1).0)
                                && pos.0 >= (s.0).0.min((s.1).0)
                                && pos.1 <= (s.0).1.max((s.1).1)
                                && pos.1 >= (s.0).1.min((s.1).1)
                        })
                        .unwrap();
                    let steps = wire.iter()
                        .take(seg_ix)
                        .map(|s| ((s.0).0 - (s.1).0).abs() + ((s.0).1 - (s.1).1).abs())
                        .sum::<isize>()
                        + ((wire[seg_ix].0).0 - pos.0).abs()
                        + ((wire[seg_ix].0).1 - pos.1).abs();
                    steps as usize
                })
                .collect::<Vec<usize>>();
            (delays[1] + delays[0]) as usize
        })
        .min()
        .unwrap();
    (manhattan, delay)
}

#[test]
fn test_0() {
    assert_eq!(run("R8,U5,L5,D3\nU7,R6,D4,L4"), (6, 30));
}

#[test]
fn test_1() {
    assert_eq!(
        run("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        (159, 610)
    );
}
