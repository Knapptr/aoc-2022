#[derive(Debug)]
pub struct Item {
    worry_level: u128,
}

impl Item {
    fn create(worry_level: u128) -> Item {
        Item { worry_level }
    }
    fn set_worry(&mut self, operation: &Operation, divisor: u128) {
        match operation {
            Operation::Add(x) => self.worry_level += x,
            Operation::Multiply(x) => self.worry_level = self.worry_level * x,
            Operation::Square => self.worry_level = self.worry_level * self.worry_level,
        }
        // self.worry_level = self.worry_level / 3; // for part 1
        self.worry_level = self.worry_level % divisor
    }
}

#[derive(Debug)]
enum Operation {
    Add(u128),
    Square,
    Multiply(u128),
}
#[derive(Debug)]
pub struct Monkey {
    items: Vec<Item>,
    pub test_number: u128,
    operation: Operation,
    pub inspection_count: u32,
    target_a: usize,
    target_b: usize,
}

impl Monkey {
    pub fn from_input(input: &str) -> Monkey {
        let mut lines = input.lines().skip(1);
        Monkey {
            items: parse_items(lines.next().unwrap()),
            operation: parse_op(lines.next().unwrap()),
            test_number: parse_test_val(lines.next().unwrap()),
            target_a: parse_target(lines.next().unwrap()),
            target_b: parse_target(lines.next().unwrap()),
            inspection_count: 0,
        }
    }
    fn create(operation: Operation, test_number: u128, target_a: usize, target_b: usize) -> Monkey {
        Monkey {
            items: Vec::new(),
            inspection_count: 0,
            operation,
            target_a,
            target_b,
            test_number,
        }
    }
    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn inspect_all_items(&mut self, divisor: u128) -> Vec<Item> {
        let mut inspected_items = Vec::new();
        while self.items.len() > 0 {
            inspected_items.push(self.inspect_last_item(divisor));
        }
        inspected_items
    }

    fn inspect_last_item(&mut self, divisor: u128) -> Item {
        let mut inspected_item = self.items.pop().expect("There was no item");
        self.inspection_count += 1;
        inspected_item.set_worry(&self.operation, divisor);
        inspected_item
    }

    pub fn get_target(&self, item: &Item) -> usize {
        if item.worry_level % self.test_number == 0 {
            self.target_a
        } else {
            self.target_b
        }
    }
}

mod test_monkeys {
    use super::*;
    #[cfg(test)]
    #[test]
    fn worry_level_set_correctly() {
        let operation = Operation::Add(2);
        let mut item = Item::create(1);
        item.set_worry(&operation, 1);
        assert_eq!(item.worry_level, 1)
    }

    #[test]
    fn throws_to_monkey() {
        let mut monkeys = Vec::new();
        let mut monkey_a = Monkey::create(Operation::Multiply(2), 3, 1, 2);
        let mut monkey_b = Monkey::create(Operation::Multiply(2), 10, 1, 2);
        let mut monkey_c = Monkey::create(Operation::Multiply(2), 10, 1, 2);

        monkey_a.items.push(Item::create(3));
        monkey_a.items.push(Item::create(2));

        monkeys.push(monkey_a);
        monkeys.push(monkey_b);
        monkeys.push(monkey_c);

        let cur_monkey = &mut monkeys[0];
        let inspected_items = cur_monkey.inspect_all_items(3);
        for item in inspected_items {
            let target = monkeys[0].get_target(&item);
            monkeys[target].catch(item);
        }

        assert_eq!(monkeys[0].items.len(), 0);
        assert_eq!(monkeys[2].items.len(), 2);
        assert_eq!(monkeys[1].items.len(), 0);
        assert_eq!(monkeys[0].inspection_count, 2);
    }
}

// fn parse_monkey(monkey_input: &str) -> Monkey { }
fn parse_items(item_input: &str) -> Vec<Item> {
    let (_, items) = item_input
        .trim()
        .split_once("items: ")
        .expect("not a valid item list");
    let item_vals: Vec<Item> = items
        .split(", ")
        .into_iter()
        .filter_map(|x| x.parse::<u128>().ok())
        .map(|x| Item::create(x))
        .collect();
    item_vals
}
fn parse_op(op_input: &str) -> Operation {
    let last_2 = op_input
        .trim()
        .split_whitespace()
        .skip(4)
        .collect::<Vec<_>>();
    let op_word = last_2
        .first()
        .expect("there is no operation sym")
        .to_string();
    let op_parse = last_2.last().expect("there is no operation value").parse();

    let op_value = match op_parse {
        Ok(v) => v,
        Err(_) => return Operation::Square,
    };

    match op_word.as_str() {
        "*" => Operation::Multiply(op_value),
        "+" => Operation::Add(op_value),
        _ => panic!("Incorrect op word"),
    }
}

fn parse_test_val(test_str: &str) -> u128 {
    test_str
        .trim()
        .split_whitespace()
        .last()
        .expect("there is a last value")
        .parse()
        .expect("error parsing value")
}

fn parse_target(target_str: &str) -> usize {
    target_str
        .trim()
        .split_whitespace()
        .last()
        .expect("there is no last value")
        .parse()
        .expect("problem parsing target value")
}

mod test_parse {
    use super::*;
    #[cfg(test)]
    #[test]
    fn parsing() {
        let op_str = "Operation: new = old + 19";
        let items_str = "Starting items: 79, 98";
        let test_str = "Test: divisible by 32";
        let target_str = "If true: throw to monkey 2";
        let op = parse_op(op_str);
        let test_val = parse_test_val(test_str);
        let target = parse_target(target_str);
        let items = parse_items(items_str);

        assert_eq!(test_val, 32);
        assert_eq!(target, 2);
        assert_eq!(items[0].worry_level, 79);
        assert_eq!(items[1].worry_level, 98);
        assert!(matches!(op, Operation::Add(19)));
    }

    #[test]
    fn parse_monkey() {
        let in_str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let monkey = Monkey::from_input(in_str);

        assert_eq!(monkey.items.len(), 2);
        assert_eq!(monkey.test_number, 23);
        assert_eq!(monkey.target_b, 3);
    }
}
