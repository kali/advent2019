use std::fs;

fn main() {
    let fuel: usize = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<usize>().unwrap() / 3 - 2)
        .sum();
    println!("fuel 1: {}", fuel);
    let fuel: usize = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| {
            let mut total = s.parse::<usize>().unwrap() / 3 - 2;
            let mut extra = total;
            loop {
                extra = (extra / 3).saturating_sub(2);
                total += extra;
                if extra == 0 {
                    break
                }
            }
            total
        })
        .sum();
    println!("fuel 2: {}", fuel);
}
