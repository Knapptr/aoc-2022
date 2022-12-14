#![allow(dead_code)]

use std::str::Lines;
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
    pub fn from_input(input: &mut Lines) -> Drive {
        let mut folders = Vec::new();
        let mut files = Vec::new();
        while let Some(line) = input.next() {
            let command = Self::parse_command(line);
            match command {
                Command::CD(dest) => match dest {
                    DirCmd::Down(dest) => {
                        folders.push(Drive::from_input(input));
                    }
                    DirCmd::Up => {
                        return Drive { folders, files };
                    }
                },
                Command::LS => (),
                Command::DIR => (),
                Command::FILE(name, size) => files.push(File { name, size }),
            }
        }
        Drive { folders, files }
    }

    pub fn get_folder_sizes(&self) -> Vec<u32> {
        let mut folder_list = Vec::new();
        folder_list.push(self.get_size());
        for folder in &self.folders {
            let res = folder.get_folder_sizes();
            for val in res {
                folder_list.push(val);
            }
        }
        folder_list
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
    let mut lines_in = TEST_IN.lines();
    let parsed = Drive::from_input(&mut lines_in);
    assert_eq!(parsed.folders.len(), 2);
}
#[test]
fn counts_size() {
    let mut lines_in = TEST_IN.lines();
    let parsed = Drive::from_input(&mut lines_in);
    assert_eq!(parsed.get_size(), 48381165);
}
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
