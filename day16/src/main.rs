fn fft(input: &[i8]) -> Vec<i8> {
    let mut output = vec![];
    for i in 1..=input.len() {
        let mut sum = 0i32;
        let pattern_len = 4 * i;
        let loops = (input.len() + pattern_len - 1) / pattern_len;
        for p in 0..loops {
            let min = p * pattern_len + i - 1;
            sum += input
                .iter()
                .skip(min)
                .take(i)
                .map(|x| *x as i32)
                .sum::<i32>();
            let min = p * pattern_len + 3 * i - 1;
            sum -= input
                .iter()
                .skip(min)
                .take(i)
                .map(|x| *x as i32)
                .sum::<i32>();
        }
        output.push((sum.abs() % 10) as i8)
    }
    output
}

fn fmt(v: &[i8]) -> String {
    v.iter()
        .map(|&b| (b as u8 + b'0') as char)
        .collect::<String>()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input
        .trim()
        .bytes()
        .map(|b| (b - b'0') as i8)
        .collect::<Vec<_>>();
    let mut values = input.clone();
    for _i in 0..100 {
        values = fft(&values);
    }

    dbg!(fmt(&values[0..8]));

    let mut full_input: Vec<i8> = vec![];
    for _ in 0..10000 {
        full_input.extend(&input);
    }

    let offset = input
        .iter()
        .take(7)
        .map(|&b| (b'0' + b as u8) as char)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    dbg!(offset);
    // for the nth value, pattern is (n-1) zeroes, then just ones
    assert!(2 * offset - 1 >= full_input.len());
    let mut values = full_input[offset..].to_vec();
    values.reverse();

    for i in 0..100 {
        dbg!(i);
        values = values
            .iter()
            .scan(0i32, |acc, x| {
                *acc += *x as i32;
                Some((acc.abs() % 10) as i8)
            })
            .collect();
    }
    values.reverse();
    dbg!(fmt(&values[0..8]));
}
