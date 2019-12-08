use itertools::Itertools;
use std::fs;

fn main() {
    let image = fs::read_to_string("input").unwrap();
    let part1 = image
        .trim()
        .bytes()
        .chunks(25 * 6)
        .into_iter()
        .map(|l| l.into_iter().collect::<Vec<u8>>())
        .min_by_key(|l| l.iter().filter(|&b| *b == b'0').count())
        .map(|l| {
            l.iter().filter(|&b| *b == b'1').count() * l.iter().filter(|&b| *b == b'2').count()
        })
        .unwrap();
    dbg!(part1);
    let data = image.trim().as_bytes();
    let layers = data.len() / 6 / 25;
    for row in 0..6 {
        for col in 0..25 {
            let color = (0..layers)
                .rev()
                .map(|l| data[col + row * 25 + l * 6 * 25] - b'0')
                .fold(2, |cur, next| [0, 1, cur][next as usize]);
            if color == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
