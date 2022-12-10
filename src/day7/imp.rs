// $ cd
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k

fn navigate_tree<'a>(root: &'a mut Directory, to: &'a str) -> Option<&'a mut Directory> {
    root.find_child(to)
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub children: Vec<Directory>,
    files: Vec<File>,
}
impl Directory {
    pub fn create(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            children: Vec::new(),
            files: Vec::new(),
        }
    }
    pub fn add_child_dir(&mut self, dir_name: &str) {
        if self.children.iter().filter(|x| x.name == dir_name).count() > 0 {
            return ();
        }
        let new_dir = Directory::create(dir_name);
        self.children.push(new_dir);
    }
    pub fn add_file(&mut self, file: File) {
        if self.files.iter().filter(|x| x.name == file.name).count() > 0 {
            return ();
        }
        self.files.push(file);
    }
    pub fn find_child(&mut self, child_name: &str) -> Option<&mut Directory> {
        if self.name == child_name {
            return Some(self);
        }
        let mut found_child: Option<&mut Directory> = None;
        for child in &mut self.children {
            match child.find_child(child_name) {
                Some(child) => found_child = Some(child),
                None => (),
            }
        }
        found_child
    }
    pub fn get_size(&self) -> u32 {
        self.files.iter().filter_map(|x| Some(x.size)).sum::<u32>() + self.get_children_size()
    }
    pub fn get_children_size(&self) -> u32 {
        self.children.iter().map(|child| child.get_size()).sum()
    }
}
#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}
#[derive(Debug)]
pub enum Command {
    CD(String),
    LS,
}

fn parse_command(string: &str) -> Command {
    let parsed_input: Vec<String> = string
        .split_whitespace()
        .skip(1)
        .take(2)
        .filter_map(|x| Some(x.to_string()))
        .collect();
    let command_string = parsed_input.first().expect("There is a command");
    let command_arg_string = parsed_input.last().expect("There is an argument");

    match command_string.as_str() {
        "ls" => Command::LS,
        "cd" => Command::CD(command_arg_string.to_owned()),
        _ => panic!(),
    }
}

fn parse_file(string: &str) -> File {
    let parsed_input: Vec<String> = string
        .split_whitespace()
        .take(2)
        .map(|x| x.to_string())
        .collect();
    let file_name = parsed_input
        .last()
        .expect("There is a file name")
        .to_string();
    let file_size: u32 = parsed_input
        .first()
        .expect("There is a file size")
        .parse()
        .expect("Unable to parse");
    File {
        name: file_name,
        size: file_size,
    }
}
#[derive(Debug)]
pub enum LineType {
    Command(Command),
    File(File),
    Dir(String),
}
fn parse_dir(string: &str) -> String {
    string
        .split_whitespace()
        .skip(1)
        .next()
        .expect("no dir name given")
        .to_string()
}
pub fn parse_line(string: &str) -> LineType {
    let first_word = string.split_whitespace().next().unwrap();
    match first_word {
        "$" => LineType::Command(parse_command(string)),
        "dir" => LineType::Dir(parse_dir(string)),
        _ => LineType::File(parse_file(string)),
    }
}

#[cfg(test)]
#[test]

fn parses_cd() {
    let cd_string = "$ cd ..";

    let parsed_command = parse_command(cd_string);

    let expected_string = "..".to_string();

    assert!(matches!(parsed_command, Command::CD(expected_string)));
}

#[test]
fn parses_ls() {
    let ls_string = "$ ls";

    let parsed_command = parse_command(ls_string);

    assert!(matches!(parsed_command, Command::LS));
}
#[test]
#[should_panic]
fn panics_on_non_command() {
    let ls_string = "$ sed";
    let parsed_command = parse_command(ls_string);
}

#[test]
fn parses_file() {
    let file_string = "25402 test.txt";
    let parsed_file = parse_file(file_string);
    assert_eq!(parsed_file.size, 25402);
    assert_eq!(parsed_file.name, "test.txt");
}

#[test]
fn create_dir() {
    let dir = Directory::create("test");

    assert_eq!(dir.name, "test");
}

#[test]
fn add_file_to_dir() {
    let mut dir = Directory::create("test");
    let file = File {
        name: String::from("filename"),
        size: 99,
    };

    dir.add_file(file);

    assert_eq!(dir.files.len(), 1);
}
#[test]
fn get_child_dir() {
    let mut dir = Directory::create("test");

    dir.add_child_dir("subdir");
    let subby = navigate_tree(&mut dir, "subdir").unwrap();
    assert_eq!(subby.name, "subdir");
}
#[test]
fn add_dir_to_subdir() {
    let mut dir = Directory::create("test");

    dir.add_child_dir("subdir");
    let sub_in_tree = navigate_tree(&mut dir, "subdir").unwrap();
    sub_in_tree.add_child_dir("sub-subdir");
    let subsub = navigate_tree(&mut dir, "sub-subdir").unwrap();
    assert_eq!(subsub.name, "sub-subdir");
}
