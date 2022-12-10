pub enum Action {
    Noop,
    Addx(i32),
}
impl Action {
    pub fn from_str(line: &str) -> Action {
        if line.starts_with("noop") {
            return Action::Noop;
        }
        let (_, add_amt) = line.split_once(" ").expect("There is not a mov number");
        Action::Addx(add_amt.parse::<i32>().expect("Error parsing input"))
    }
}

pub struct CPU {
    register_value: i32,
    report_start: u32,
    report_interval: u32,
    display_row_length: u32,
    current_tick: u32,
    display_buffer: String,
    tick_reports: Vec<(u32, i32)>,
    pub display_output: Vec<String>,
}
impl CPU {
    pub fn create(report_start: u32, report_interval: u32, display_row_length: u32) -> CPU {
        CPU {
            register_value: 1,
            current_tick: 0,
            tick_reports: Vec::new(),
            display_buffer: String::new(),
            display_output: Vec::new(),
            report_interval,
            report_start,
            display_row_length,
        }
    }

    pub fn render(&self) -> String {
        self.display_output.join("\n")
    }
    fn render_to_buffer(&mut self) {
        self.display_buffer += &self.should_render();
    }
    fn render_row(&mut self) {
        if self.current_tick > 1
            && self.current_tick % self.display_row_length == self.display_row_length - 1
        {
            self.display_output.push(self.display_buffer.clone());
            self.display_buffer.clear();
        }
    }
    fn should_render(&self) -> String {
        let sprite_center = self.register_value % self.display_row_length as i32;
        let sprite_max = sprite_center + 1;
        let sprite_min = sprite_center - 1;
        let current_row_index = self.current_tick % self.display_row_length;

        if (current_row_index as i32) < sprite_min || (current_row_index as i32) > sprite_max {
            return ".".to_string();
        }
        return "#".to_string();
    }
    fn signal_strength(&self) -> i32 {
        self.current_tick as i32 * self.register_value
    }
    fn report(&mut self) {
        if self.current_tick < self.report_start {
            return ();
        }
        if self.current_tick == self.report_start
            || ((self.current_tick - self.report_start) % self.report_interval == 0)
        {
            self.tick_reports
                .push((self.current_tick, self.signal_strength()));
        }
    }
    fn tick(&mut self, amt: u32) {
        for _ in 0..amt {
            self.render_to_buffer();
            self.render_row();
            self.current_tick += 1;
            self.report();
        }
    }
    pub fn process_action(&mut self, action: Action) {
        match action {
            Action::Noop => self.tick(1),
            Action::Addx(val) => {
                self.tick(2);
                self.register_value += val;
            }
        }
    }
    pub fn get_report(&self) -> &Vec<(u32, i32)> {
        &self.tick_reports
    }
}

// Tests

#[cfg(test)]
#[test]
fn parses_action() {
    let noop_input = "noop";
    let addx_input = "addx 39";
    let parsed_noop = Action::from_str(noop_input);
    let parsed_addx = Action::from_str(addx_input);
    assert!(matches!(parsed_noop, Action::Noop));
    assert!(matches!(parsed_addx, Action::Addx(39)));
}

#[test]
fn gets_report() {
    let mut cpu = CPU::create(20, 40, 40);
    for line in TEST_INPUT.lines() {
        let action = Action::from_str(line);
        cpu.process_action(action);
    }
    println!("Reports: {:?}", cpu.get_report());
    let sum: i32 = cpu.get_report().iter().map(|x| x.1).sum();

    assert_eq!(sum, 13140);
}
#[allow(dead_code)]
const TEST_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
