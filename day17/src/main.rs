fn main() {
    let mut vm = intcode::Machine::new_from_file("input");
    vm.run(&[]);
    let output = vm
        .outputs
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<String>();
    dbg!(alignment(&output));
    let p = path(&output);
    let c = compact(&p);

    let mut vm = intcode::Machine::new_from_file("input");
    vm.mem[0] = 2;
    for s in &[ &*c.0, &*c.1, &*c.2, &*c.3, "n" ] {
        println!("sending s: {}", s);
        while !vm.waiting() {
            vm.step()
        }
        vm.inputs.extend(s.chars().map(|c| c as isize));
        vm.inputs.push('\n' as isize);
    }
    while !vm.done() {
        vm.step()
    }
    dbg!(vm.outputs);
}

fn alignment(grid: &str) -> usize {
    let grid = grid
        .trim()
        .lines()
        .map(|s| s.as_bytes())
        .collect::<Vec<&[u8]>>();
    let width = grid[0].len();
    let height = grid.len();
    let mut alignment = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if grid[y][x] == b'#'
                && grid[y - 1][x] == b'#'
                && grid[y + 1][x] == b'#'
                && grid[y][x - 1] == b'#'
                && grid[y][x + 1] == b'#'
            {
                alignment += x * y;
            }
        }
    }
    alignment
}

fn path(input: &str) -> Vec<String> {
    let grid = input
        .trim()
        .lines()
        .map(|s| s.as_bytes())
        .collect::<Vec<&[u8]>>();
    let width = grid[0].len();
    let height = grid.len();
    let mut bot = grid
        .iter()
        .enumerate()
        .filter_map(|(y, l)| l.iter().position(|&b| b == b'^').map(|x| (x, y)))
        .next()
        .unwrap();
    let inc = [(0isize, -1isize), (-1, 0), (0, 1), (1, 0)];
    let mut dir = 0; // N W S E
    let mut seq: Vec<String> = vec![];
    loop {
        if dir % 2 == 0 {
            if bot.0 > 0 && grid[bot.1][bot.0 - 1] == b'#' {
                seq.push((if dir == 2 { "R" } else { "L" }).to_string());
                dir = 1;
            } else if bot.0 < width && grid[bot.1][bot.0 + 1] == b'#' {
                seq.push((if dir == 0 { "R" } else { "L" }).to_string());
                dir = 3;
            } else {
                break;
            }
        } else {
            if bot.1 > 0 && grid[bot.1 - 1][bot.0] == b'#' {
                seq.push((if dir == 1 { "R" } else { "L" }).to_string());
                dir = 0;
            } else if bot.1 < width && grid[bot.1 + 1][bot.0] == b'#' {
                seq.push((if dir == 3 { "R" } else { "L" }).to_string());
                dir = 2;
            } else {
                break;
            }
        }
        let dist = (1isize..)
            .take_while(|n| {
                let x = bot.0 as isize + inc[dir].0 * n;
                let y = bot.1 as isize + inc[dir].1 * n;
                x >= 0
                    && x < width as isize
                    && y >= 0
                    && y < height as isize
                    && grid[y as usize][x as usize] == b'#'
            })
            .last()
            .unwrap();
        bot.0 = (bot.0 as isize + inc[dir].0 * dist) as usize;
        bot.1 = (bot.1 as isize + inc[dir].1 * dist) as usize;
        seq.push(format!("{}", dist));
    }
    seq
}

fn compact(seq: &[String]) -> (String, String, String, String) {
    let seq = seq.join(",").to_string();
    dbg!(&seq);
    for len_a in 1..seq.len().min(20) {
        let a: String = seq.chars().take(len_a).collect();
        if a.chars().last().unwrap() == ',' {
            continue;
        }
        let seq_rep_a = seq.replace(&a, "A").to_string();
        let start_b = seq_rep_a.chars().position(|c| !"A,".contains(c)).unwrap();
        for len_b in 1..(seq_rep_a.len() - start_b).min(20) {
            let b: String = seq_rep_a.chars().skip(start_b).take(len_b).collect();
            if b.chars().last().unwrap() == ',' {
                continue;
            }
            if b.chars().last().unwrap() == 'A' {
                break;
            }
            let seq_rep_b = seq_rep_a.replace(&b, "B");
            let start_c = seq_rep_b.chars().position(|c| !"AB,".contains(c)).unwrap();
            let len_c = seq_rep_b
                .chars()
                .skip(start_c)
                .take_while(|&c| !"AB".contains(c))
                .count();
            let c: String = seq_rep_b.chars().skip(start_c).take(len_c).collect();
            let c = c.trim_end_matches(",");
            let seq_rep = seq_rep_b.replace(&c, "C");
            if seq_rep.chars().all(|c| "ABC,".contains(c)) {
                if a.len() <= 20 && b.len() <= 20 && c.len() <= 20 && seq_rep.len() <= 20 {
                    return (seq_rep, a, b, c.to_string());
                }
            }
        }
    }
    panic!()
}

#[test]
fn test_1() {
    let a = alignment(
        r#"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."#,
    );
    assert_eq!(a, 76);
}

#[test]
fn test_2() {
    let grid = r#"#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......"#;
    let p = path(grid);
    assert_eq!(
        p.join(","),
        "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
    );
    let c = compact(&p);
}
