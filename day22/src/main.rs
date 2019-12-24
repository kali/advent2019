enum Op {
    New,
    Cut(isize),
    Inc(usize),
}

impl Op {
    fn apply(&self, n: usize, ix: usize) -> usize {
        match self {
            Op::New => n - ix - 1,
            Op::Cut(cut) => ((ix + n) as isize - cut) as usize % n,
            Op::Inc(inc) => ((*inc as u128 * ix as u128) % n as u128) as usize,
        }
    }

    fn inverse(&self, n: usize) -> Op {
        match self {
            Op::New => Op::New,
            Op::Cut(cut) => Op::Cut(-cut),
            Op::Inc(inc) => {
                Op::Inc(modinverse::modinverse(*inc as i128, n as i128).unwrap() as usize)
            }
        }
    }

    fn as_affine(&self) -> Affine {
        match self {
            Op::New => Affine {
                mul: -1,
                offset: -1,
            },
            Op::Cut(cut) => Affine {
                mul: 1,
                offset: -cut as _,
            },
            Op::Inc(inc) => Affine {
                mul: *inc as _,
                offset: 0,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Affine {
    mul: i128,
    offset: i128,
}

impl Affine {
    fn compose(&self, n: usize, other: &Affine) -> Affine {
        // f(x) = ax+b, g(x) = cx+d
        // g(f(x)) = c.f(x)+d = cax + bc + d
        let mul = (self.mul * other.mul) % n as i128;
        let offset = (self.offset * other.mul + other.offset) % n as i128;
        Affine { mul, offset }
    }

    fn apply(&self, n: usize, ix: usize) -> usize {
        let n = n as i128;
        ((((self.mul * ix as i128 + self.offset) % n) + n) % n) as usize
    }

    fn inverse(&self, n: usize) -> Affine {
        let n = n as i128;
        // y = a.x + b, x = y/a - b/a
        dbg!(self.mul);
        let a_inv = modinverse::modinverse(self.mul.abs(), n).unwrap();
        let a_inv = a_inv * self.mul.signum();
        Affine {
            mul: ((a_inv % n) + n) % n,
            offset: (((-self.offset * a_inv) % n) + n) % n,
        }
    }

    fn ladder(&self, n: usize, mut v: usize, mut exp: usize) -> usize {
        let mut affine = *self;
        while exp != 0 {
            if exp & 1 == 1 {
                v = affine.apply(n, v);
            }
            exp >>= 1;
            affine = affine.compose(n, &affine);
        }
        v
    }
}

fn parse(shuffle: &str) -> Vec<Op> {
    shuffle
        .trim()
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Op::New
            } else if line.starts_with("cut") {
                Op::Cut(line[4..].parse().unwrap())
            } else if line.starts_with("deal with increment") {
                Op::Inc(line[20..].parse().unwrap())
            } else {
                panic!()
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let shuffle = parse(&input);
    let affine = shuffle[1..].iter().fold(shuffle[0].as_affine(), |af, op| {
        af.compose(10007, &op.as_affine())
    });
    dbg!(shuffle.iter().fold(2019, |ix, op| op.apply(10007, ix)));
    dbg!(affine.apply(10007, 2019));

    let n = 119315717514047usize;
    let affine = shuffle[1..].iter().fold(shuffle[0].as_affine(), |af, op| {
        af.compose(n, &op.as_affine())
    });
    let rev = affine.inverse(n);
    dbg!(rev.ladder(n, 2020, 101741582076661));
}

#[test]
fn t_new() {
    let mut result = vec![-1isize; 10];
    for i in 0..10 {
        result[Op::New.apply(10, i)] = i as isize;
    }
    assert_eq!(&result, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn t_cut() {
    let mut result = vec![-1isize; 10];
    for i in 0..10 {
        result[Op::Cut(3).apply(10, i)] = i as isize;
    }
    assert_eq!(&result, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
}

#[test]
fn t_cut_neg() {
    let mut result = vec![-1isize; 10];
    for i in 0..10 {
        result[Op::Cut(-4).apply(10, i)] = i as isize;
    }
    assert_eq!(&result, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn t_inc() {
    let mut result = vec![-1isize; 10];
    for i in 0..10 {
        result[Op::Inc(3).apply(10, i)] = i as isize;
    }
    assert_eq!(&result, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
}

#[test]
fn t_follow_2() {
    let mut result = vec![-1isize; 10];
    let shuffle = parse(
        r#"cut 6
deal with increment 7
deal into new stack"#,
    );
    for i in 0..10 {
        let pos = shuffle.iter().fold(i, |ix, op| op.apply(10, ix));
        result[pos] = i as isize;
    }
    assert_eq!(&result, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
}

#[test]
fn t_follow_2_compose() {
    let mut result = vec![-1isize; 10];
    let shuffle = parse(
        r#"cut 6
deal with increment 7
deal into new stack"#,
    );
    let affine = shuffle[1..].iter().fold(shuffle[0].as_affine(), |af, op| {
        af.compose(10, &op.as_affine())
    });
    dbg!(affine);
    for i in 0..10 {
        result[affine.apply(10, i)] = i as isize;
    }
    assert_eq!(&result, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
}

#[test]
fn t_ladder() {
    let n = 97;
    let exp = 19;
    let mut v = 17;
    let affine = Affine { mul: 5, offset: -2 };
    for i in 0..exp {
        v = affine.apply(n,v );
    }
    let v2 = affine.ladder(n, 17, 19);
    assert_eq!(v, v2);
}
