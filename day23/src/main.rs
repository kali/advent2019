fn main() {
    let vm = intcode::Machine::new_from_file("input");
    let mut hosts = (0..50).map(|i| {
        let mut vm = vm.clone();
        vm.inputs.push(i);
        vm
    }).collect::<Vec<_>>();

    'l: loop {
        for h in 0..50 {
            hosts[h].inputs.push(-1);
            hosts[h].steps();
            while hosts[h].outputs.len() >= 3 {
                let dest = hosts[h].outputs.remove(0) as usize;
                let x = hosts[h].outputs.remove(0);
                let y = hosts[h].outputs.remove(0);

                if dest == 255 {
                    dbg!(y);
                    break 'l;
                }
                hosts[dest].inputs.push(x);
                hosts[dest].inputs.push(y);
            }
        }
    }

    let mut hosts = (0..50).map(|i| {
        let mut vm = vm.clone();
        vm.inputs.push(i);
        vm
    }).collect::<Vec<_>>();

    let mut prev_y = -2;
    let mut nat = (-1, -1);
    'l2: loop {
        for h in 0..50 {
            hosts[h].inputs.push(-1);
            hosts[h].steps();
            while hosts[h].outputs.len() >= 3 {
                let dest = hosts[h].outputs.remove(0) as usize;
                let x = hosts[h].outputs.remove(0);
                let y = hosts[h].outputs.remove(0);

                if dest == 255 {
                    nat = (x,y);
                } else {
                    hosts[dest].inputs.push(x);
                    hosts[dest].inputs.push(y);
                }
            }
        }
        if hosts.iter().all(|h| h.inputs.len() == 0) {
            if nat.1 == prev_y {
                dbg!(prev_y);
                break 'l2;
            }
            prev_y = nat.1;
            hosts[0].inputs.push(nat.0);
            hosts[0].inputs.push(nat.1);
        }
    }
}
