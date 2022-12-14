#![allow(dead_code)]
pub const TEST_IN: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[derive(Debug)]
pub struct File {
    size: u32,
    name: String,
}
#[derive(Debug)]
pub struct Drive {
    pub folders: Vec<Drive>,
    pub files: Vec<File>,
}
pub enum Command {
    CD(DirCmd),
    LS,
    DIR,
    FILE(String, u32),
}
pub enum DirCmd {
    Up,
    Down(String),
}
impl Drive {
    fn init() -> Drive {
        Drive {
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
    pub fn get_size(&self) -> u32 {
        let child_files_size: u32 = self.files.iter().map(|f| f.size).sum();
        let child_dirs_size: u32 = self.folders.iter().map(|fol| fol.get_size()).sum();
        child_files_size + child_dirs_size
    }
    // make this recursive
    pub fn from_input(input: &str) -> Drive {
        let mut drive_stack: Vec<Drive> = Vec::new();
        for line in input.lines() {
            let command = Self::parse_command(line);
            match command {
                Command::CD(dest) => match dest {
                    DirCmd::Down(dest) => {
                        drive_stack.push(Drive::init());
                    }
                    DirCmd::Up => {
                        let lower_drive = drive_stack.pop().expect("no stack to pop");
                        drive_stack
                            .last_mut()
                            .expect("no drive to push onto")
                            .folders
                            .push(lower_drive);
                    }
                },
                Command::LS => (),
                Command::DIR => (),
                Command::FILE(name, size) => drive_stack
                    .last_mut()
                    .expect("nothing on stack")
                    .files
                    .push(File { name, size }),
            }
        }
        while drive_stack.len() > 1 {
            let child_drive = drive_stack.pop().unwrap();
            drive_stack
                .last_mut()
                .expect("There was no drive to push into")
                .folders
                .push(child_drive);
        }
        drive_stack.pop().unwrap()
    }

    fn parse_command(input: &str) -> Command {
        let (first_word, other_words) = input
            .split_once(" ")
            .expect("There were not at least 2 words");
        match first_word {
            "$" => match other_words {
                "ls" => Command::LS,
                _ => {
                    let (cmd, dest) = other_words.split_once(" ").unwrap();
                    match dest {
                        ".." => Command::CD(DirCmd::Up),
                        _ => Command::CD(DirCmd::Down(dest.to_string())),
                    }
                }
            },
            "dir" => Command::DIR,
            _ => {
                let size: u32 = first_word.parse().expect("error parsing number");
                let name = other_words.to_string();
                Command::FILE(name, size)
            }
        }
    }
}

#[cfg(test)]
#[test]
fn parses_drive() {
    let parsed = Drive::from_input(TEST_IN);
    assert_eq!(parsed.folders.len(), 2);
}
#[test]
fn counts_size() {
    let parsed = Drive::from_input(TEST_IN);
    assert_eq!(parsed.get_size(), 48381165)
}
