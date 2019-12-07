use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    dbg!(part_1(&*input));
    dbg!(part_2(&*input));
}

fn part_1(input: &[isize]) -> isize {
    let machine = intcode::Machine::new(input.to_vec());
    (0..5)
        .permutations(5)
        .map(|settings| {
            settings.iter().fold(0, |input, setting| {
                machine.clone().run(&[*setting, input])[0]
            })
        })
        .max()
        .unwrap()
}

fn part_2(input: &[isize]) -> isize {
    (5..10)
        .permutations(5)
        .map(|settings| {
            let mut machines = settings
                .iter()
                .map(|setting| {
                    let mut m = intcode::Machine::new(input.to_vec());
                    m.inputs.push(*setting);
                    m
                })
                .collect::<Vec<_>>();
            let mut signal = 0;
            let mut e_signal = 0;
            loop {
                for m in &mut machines {
                    m.inputs.push(signal);
                    while m.outputs.is_empty() {
                        if m.done() {
                            return e_signal;
                        }
                        m.step();
                    }
                    signal = m.outputs.remove(0);
                }
                e_signal = signal;
            }
        })
        .max()
        .unwrap()
}

#[test]
fn t1() {
    assert_eq!(
        part_1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
        43210
    );
}

#[test]
fn t2() {
    assert_eq!(
        part_2(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ]),
        139629729
    );
}
