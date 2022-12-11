use super::imp::Monkey;

pub fn solve(input: &str) {
    let monkey_inputs = input.split("\n\n");

    let mut monkeys = Vec::new();
    for mk in monkey_inputs {
        monkeys.push(Monkey::from_input(mk));
    }

    let gcd: u128 = monkeys.iter().map(|x| x.test_number).product();
    for _round in 0..10_000 {
        for m_i in 0..monkeys.len() {
            let inspected_items = monkeys[m_i].inspect_all_items(gcd);
            for item in inspected_items {
                let target = monkeys[m_i].get_target(&item);
                monkeys[target].catch(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    let last_2: Vec<u32> = monkeys
        .drain(monkeys.len() - 2..)
        .map(|x| x.inspection_count)
        .collect();

    let monkey_business: u128 = last_2[0] as u128 * last_2[1] as u128;

    println!("Part 1: {}", monkey_business);
}
