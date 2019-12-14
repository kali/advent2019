fn ore_for(quantity: usize, rules: &Vec<Vec<(usize, String)>>) -> usize {
    let mut need = std::collections::HashMap::new();
    need.insert("FUEL", quantity as isize);
    while let Some((wanted, q)) = need
        .iter()
        .find(|(k, v)| **k != "ORE" && **v > 0)
        .map(|(k, v)| (*k, *v))
        .clone()
    {
        let rule = rules
            .iter()
            .find(|r| &r.last().unwrap().1 == wanted)
            .unwrap();
        let quantum = rule.iter().last().unwrap().0 as isize;
        let iterations = (q + quantum - 1) / quantum;
        for r in rule.iter().rev().skip(1) {
            *need.entry(&r.1).or_insert(0isize) += r.0 as isize * iterations;
        }
        *need.get_mut(wanted).unwrap() -= quantum * iterations;
    }
    need["ORE"] as usize
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let rules: Vec<Vec<(usize, String)>> = input
        .lines()
        .map(|line| {
            line.replace(" => ", ", ")
                .split(",")
                .map(|x| {
                    let mut tokens = x.trim().split(" ");
                    (
                        tokens.next().unwrap().parse::<usize>().unwrap(),
                        tokens.next().unwrap().to_string(),
                    )
                })
                .collect()
        })
        .collect();
    let ore_for_one_fuel = ore_for(1, &rules);
    dbg!(ore_for_one_fuel);
    let trillion = 1_000_000_000_000;
    let mut min = trillion / ore_for_one_fuel;
    let mut max = min * 2;
    assert!(ore_for(max, &rules) > trillion);
    while min + 1 < max {
        let half = (max + min) / 2;
        if ore_for(half, &rules) > trillion {
            max = half;
        } else {
            min = half;
        }
    }
    dbg!(min);
}
