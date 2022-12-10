use super::imp::{Action, CPU};

pub fn solve(input: &str) {
    let mut cpu = CPU::create(20, 40, 40);

    for line in input.lines() {
        let action = Action::from_str(line);
        cpu.process_action(action);
    }

    let report_sum: i32 = cpu.get_report().iter().map(|x| x.1).sum();
    println!("Part 1: {:?}", report_sum);
    println!("Part 2: \n{}", cpu.render())
}
