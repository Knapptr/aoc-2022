#[derive(Debug)]
pub struct Elf {
    snacks: Vec<u32>,
}

impl Elf {
    pub fn init() -> Elf {
        Elf { snacks: Vec::new() }
    }

    pub fn add_snack(&mut self, cal_count: u32) {
        self.snacks.push(cal_count)
    }

    pub fn calorie_total(&self) -> u32 {
        self.snacks.iter().sum()
    }

    pub fn clone(&self) -> Elf {
        Elf {
            snacks: self.snacks.clone(),
        }
    }
}

pub struct Elves {
    list: Vec<Elf>,
}
impl Elves {
    pub fn new() -> Elves {
        Elves { list: Vec::new() }
    }
    fn sort_by_calories(&mut self) {
        self.list.sort_by_key(|e| e.calorie_total())
    }

    pub fn get_and_print_top_3(&mut self) {
        self.sort_by_calories();
        let most = self.list.pop().unwrap().calorie_total();
        let most2 = self.list.pop().unwrap().calorie_total();
        let most3 = self.list.pop().unwrap().calorie_total();
        println!(
            "Most: {}\nSecond Most: {}\nThird Most: {}",
            most, most2, most3
        )
    }
    pub fn add_elf(&mut self, elf: &Elf) {
        self.list.push(elf.clone());
    }
}
