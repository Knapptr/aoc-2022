#[derive(Debug)]
enum Operation {
    Add(u128),
    Square,
    Multiply(u128),
}

#[derive(Debug)]
pub struct Item {
    worry_level: u128,
}

impl Item {
    fn create(worry_level: u128) -> Item {
        Item { worry_level }
    }
    fn set_worry<T: Fn(u128) -> u128>(&mut self, operation: &Operation, f: T) {
        match operation {
            Operation::Add(x) => self.worry_level += x,
            Operation::Multiply(x) => self.worry_level = self.worry_level * x,
            Operation::Square => self.worry_level = self.worry_level * self.worry_level,
        }
        self.worry_level = f(self.worry_level);
    }
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
    pub fn parse(input: &str) -> Monkey {
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

    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn inspect_all_items<T: Fn(u128) -> u128>(&mut self, f: T) -> Vec<Item> {
        let mut inspected_items = Vec::new();
        while self.items.len() > 0 {
            inspected_items.push(self.inspect_last_item(&f));
        }
        inspected_items
    }

    fn inspect_last_item<T: Fn(u128) -> u128>(&mut self, f: T) -> Item {
        let mut inspected_item = self.items.pop().expect("There was no item");
        self.inspection_count += 1;
        inspected_item.set_worry(&self.operation, f);
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
