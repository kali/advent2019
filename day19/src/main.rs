fn main() {
    let vm = intcode::Machine::new_from_file("input");
    let b = |x: usize, y: usize| -> bool {
        let mut vm = vm.clone();
        vm.run(&[x as isize, y as isize]);
        vm.outputs[0] == 1
    };
    let mut p1 = 0;
    for y in 0..50 {
        for x in 0..50 {
            let b = b(x, y);
            p1 += b as usize;
            print!("{}", if b { "#" } else { "." });
        }
        println!("");
    }
    dbg!(p1);
    let fit = |top: usize| -> bool {
        let bottom = top + 99;
        let left = (0..).find(|&x| b(x, bottom)).unwrap();
        let right = left + 99;
        b(right, top)
    };
    let mut min = 1;
    let mut max = (1i32..).map(|n| 1 << n).find(|&n| fit(n as usize)).unwrap();
    while max > min + 1 {
        let t = (max + min) / 2;
        if fit(t) {
            max = t;
        } else {
            min = t;
        }
    }
    let top = max;
    let left = (0..).find(|&x| b(x, max + 99)).unwrap();
    dbg!(top + 10000 * left);
}
