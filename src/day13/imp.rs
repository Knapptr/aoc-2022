use core::fmt;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Eq)]
pub enum PacketMember {
    Val(u32),
    SubList(Vec<PacketMember>),
}

// Todo - Convert from stack to recursive
impl PacketMember {
    pub fn create_from_input(input: &str) -> PacketMember {
        let mut stack: Vec<_> = Vec::new();

        let mut chars_iter = input.chars().peekable();
        while let Some(mut char) = chars_iter.next() {
            if char == ',' {
                continue;
            }
            if char == '[' {
                stack.push(vec![]);
                continue;
            }
            if char == ']' {
                if stack.len() == 1 {
                    break;
                }
                let finished_vec = stack.pop().expect("no item in stack");
                stack
                    .last_mut()
                    .expect("no stack el")
                    .push(PacketMember::SubList(finished_vec));
                continue;
            }
            let mut num_str = String::new();
            num_str.push(char);
            let mut next_char = chars_iter.peek().unwrap();
            while next_char != &',' && next_char != &']' {
                char = chars_iter.next().unwrap();
                num_str.push(char);
                next_char = chars_iter.peek().unwrap();
            }
            stack
                .last_mut()
                .expect("no stack element")
                .push(PacketMember::Val(num_str.parse::<u32>().unwrap()));
        }
        PacketMember::SubList(stack.pop().expect("No stack to return!"))
    }
    pub fn create_marker_2() -> PacketMember {
        PacketMember::create_from_input("[[2]]")
    }
    pub fn create_marker_6() -> PacketMember {
        PacketMember::create_from_input("[[6]]")
    }
}

impl Display for PacketMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketMember::Val(int) => write!(f, "{}", int),
            PacketMember::SubList(list) => {
                let mut strs = Vec::new();
                for member in list {
                    strs.push(format!("{}", member))
                }
                return write!(f, "[{}]", strs.join(","));
            }
        }
    }
}

impl PartialEq for PacketMember {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Val(self_val), Self::Val(rhs_val)) => self_val == rhs_val,
            (Self::Val(self_val), Self::SubList(rhs_list)) => {
                vec![Self::Val(*self_val)] == *rhs_list
            }
            (Self::SubList(self_list), Self::Val(rhs_val)) => {
                *self_list == vec![Self::Val(*rhs_val)]
            }
            (Self::SubList(self_list), Self::SubList(rhs_list)) => self_list == rhs_list,
        }
    }
}

impl PartialOrd for PacketMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (PacketMember::Val(self_val), PacketMember::Val(other_val)) => {
                if self_val == other_val {
                    return Some(Ordering::Equal);
                }
                if self_val < other_val {
                    return Some(Ordering::Less);
                }
                Some(Ordering::Greater)
            }
            (PacketMember::Val(self_val), PacketMember::SubList(other_list)) => {
                Some(list_helper(&vec![PacketMember::Val(*self_val)], other_list))
            }
            (PacketMember::SubList(self_list), PacketMember::SubList(other_list)) => {
                Some(list_helper(self_list, other_list))
            }
            (PacketMember::SubList(self_list), PacketMember::Val(other_val)) => Some(list_helper(
                &self_list,
                &vec![PacketMember::Val(*other_val)],
            )),
        }
    }
}
impl Ord for PacketMember {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
fn list_helper(self_list: &Vec<PacketMember>, other_list: &Vec<PacketMember>) -> Ordering {
    let mut idx = 0;
    while idx < self_list.len() && idx < other_list.len() {
        if self_list[idx] < other_list[idx] {
            return Ordering::Less;
        }
        if self_list[idx] > other_list[idx] {
            return Ordering::Greater;
        }
        idx += 1;
    }
    if self_list.len() == other_list.len() {
        return Ordering::Equal;
    }
    if idx < self_list.len() {
        return Ordering::Greater;
    }
    if idx < other_list.len() {
        return Ordering::Less;
    }
    unreachable!();
}

pub fn display_packet(packet: &Vec<PacketMember>) {
    let mut strs = Vec::new();
    for el in packet {
        strs.push(format!("{}", el));
    }
    let str = strs.join(",");
    println!("[{}]", str);
}
#[cfg(test)]
#[test]
fn compare() {
    let packet_1_explicit = PacketMember::SubList(vec![
        PacketMember::Val(1),
        PacketMember::Val(1),
        PacketMember::Val(3),
    ]);
    let packet_2_explicit =
        PacketMember::SubList(vec![PacketMember::SubList(vec![PacketMember::Val(2)])]);

    assert!(packet_1_explicit < packet_2_explicit);
    assert!(packet_2_explicit > packet_1_explicit);
}
