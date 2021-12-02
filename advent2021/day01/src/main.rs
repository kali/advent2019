use itertools::Itertools;

fn main() {
    let values = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i64>>();
    let one = values.iter().tuple_windows().filter(|(a, b)| a < b).count();
    dbg!(one);
    let two = values
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    dbg!(two);
}
