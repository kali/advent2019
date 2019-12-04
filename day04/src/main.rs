fn check_1(n: &usize) -> bool {
    let figures: Vec<usize> = (0..6).map(|p| (n / (10usize.pow(p))) % 10).collect();
    figures.windows(2).any(|pair| pair[0] == pair[1])
        && figures.windows(2).all(|pair| pair[0] >= pair[1])
}

fn check_2(n: &usize) -> bool {
    let mut figures: Vec<usize> = (0..6).map(|p| (n / (10usize.pow(p))) % 10).collect();
    figures.insert(0, 10);
    figures.push(0);
    figures.windows(2).all(|pair| pair[0] >= pair[1])
        && figures
            .windows(4)
            .any(|quad| quad[0] != quad[1] && quad[1] == quad[2] && quad[1] != quad[3])
}

fn main() {
    let input = (136818, 685979);
    let count = (input.0..=input.1).filter(check_1).count();
    dbg!(count);
    let count = (input.0..=input.1).filter(check_2).count();
    dbg!(count);
}

#[test]
fn t1() {
    assert!(check_1(&123455));
    assert!(check_1(&111111));
    assert!(!check_1(&223450));
    assert!(!check_1(&123789));
}
