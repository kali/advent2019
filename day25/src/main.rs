fn main() {
    let mut vm = intcode::Machine::new_from_file("input");
    let commands = vec![
        "north",
        "west",
        "take planetoid",
        "west",
        "take spool of cat6",
        "east",
        "east",
        "south",
        "west",
        "north",
        "take dark matter",
        "south",
        "east",
        "east",
        "north",
        "take sand",
        "west",
        "take coin",
        "north",
        "take jam",
        "south",
        "west",
        "south",
        "take wreath",
        "west",
        "take fuel cell",
        "east",
        "north",
        "north",
        "west",
        "inv",
        "south",
    ];
    let items = &[
        "jam",
        "fuel cell",
        "planetoid",
        "sand",
        "spool of cat6",
        "coin",
        "dark matter",
        "wreath",
    ];
    for command in &commands {
        vm.steps_then_dump();
        vm.inputln(command);
    }
    for combi in 0..256 {
        for item in 0..8 {
            let verb = if combi & (1 << item) == 0 { "drop" }  else { "take" };
            vm.inputln(format!("{} {}", verb, items[item]));
            vm.steps_then_dump();
        }
        vm.inputln("south");
        vm.steps_then_dump();
    }
}
