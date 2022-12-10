// use super::imp::{parse_line, Command, Directory, LineType};
#[allow(unused_variables)]
pub fn solve(input: &str) {
    //let mut root: Directory = Directory::create("/");
    //let mut nav_stack: Vec<&mut Directory> = Vec::new();
    //nav_stack.push(&mut root);

    //for line in input.lines().skip(2) {
    //    let parsed_line = parse_line(line);
    //    match parsed_line {
    //        LineType::Command(Command::CD(cd_to)) => match cd_to.as_str() {
    //            "/" => {
    //                // nav_stack.clear();
    //            }
    //            ".." => {
    //                nav_stack.pop();
    //            }
    //            dir_name => {
    //                //check for current directory
    //                let current_dir = nav_stack.pop().expect("no stack item");
    //                let new_location = current_dir.find_child(dir_name).expect("uh oh");
    //                nav_stack.push(current_dir);
    //                nav_stack.push(new_location);
    //            }
    //        },
    //        LineType::Command(Command::LS) => (),

    //        LineType::Dir(dir_name) => {
    //            let current_dir = nav_stack.pop().expect("no stack item");
    //            current_dir.add_child_dir(&dir_name);
    //            nav_stack.push(current_dir);
    //        }
    //        LineType::File(file) => {
    //            let current_dir = nav_stack.pop().expect("no stack item");
    //            current_dir.add_file(file);
    //            nav_stack.push(current_dir);
    //        }
    //    }
    //}
    //// println!("{:#?}", root.get_size());
    //println!("{:?}", get_recursive_sizes(&root));
    //println!("{:?}", root.children[1].children);
    //fn get_recursive_sizes(dir: &Directory) -> Vec<(String, u32)> {
    //    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    //    for child in &dir.children {
    //        dir_sizes.push((child.name.to_string(), child.get_size()))
    //    }
    //    dir_sizes
    //}
}
