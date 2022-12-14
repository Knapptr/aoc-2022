use core::fmt;
use std::fmt::Display;

type PacketPair = (Vec<PacketMember>, Vec<PacketMember>);

pub fn compare_packet_pairs(packet_pair: &PacketPair) -> Comp {
    let (left, right) = packet_pair;
    let mut idx = 0;
    let mut comp_val = Comp::Eq;
    while idx < left.len() && idx < right.len() && matches!(comp_val, Comp::Eq) {
        let l_item = &left[idx];
        let r_item = &right[idx];
        comp_val = l_item.compare(r_item);
        match comp_val {
            Comp::Ordered => break,
            Comp::NotOrdered => break,
            _ => (),
        }
        idx += 1;
    }
    if idx >= left.len() && idx < right.len() {
        comp_val = Comp::Ordered
    }
    if idx < left.len() && idx >= right.len() {
        comp_val = Comp::NotOrdered
    }
    comp_val
}

#[derive(Debug)]
pub enum Comp {
    Eq,
    Ordered,
    NotOrdered,
}

#[derive(Debug, Clone)]
pub enum PacketMember {
    Val(u32),
    SubList(Vec<PacketMember>),
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
impl PacketMember {
    pub fn create_marker_2() -> Vec<PacketMember> {
        parse_packet("[[2]]")
    }
    pub fn create_marker_6() -> Vec<PacketMember> {
        parse_packet("[[6]]")
    }
    pub fn compare(&self, other: &PacketMember) -> Comp {
        match (self, other) {
            (PacketMember::Val(self_val), PacketMember::Val(other_val)) => {
                if self_val == other_val {
                    return Comp::Eq;
                };
                if self_val < other_val {
                    return Comp::Ordered;
                };
                Comp::NotOrdered
            }
            (PacketMember::Val(self_val), PacketMember::SubList(other_list)) => {
                let self_as_list = PacketMember::SubList(vec![PacketMember::Val(*self_val)]);
                self_as_list.compare(other)
            }
            (PacketMember::SubList(self_list), PacketMember::Val(other_val)) => {
                self.compare(&PacketMember::SubList(vec![PacketMember::Val(*other_val)]))
            }
            (PacketMember::SubList(self_list), PacketMember::SubList(other_list)) => {
                let mut idx = 0;
                while idx < self_list.len() && idx < other_list.len() {
                    let l_item = self_list[idx].clone();
                    let r_item = other_list[idx].clone();
                    let comp_result = l_item.compare(&r_item);
                    match comp_result {
                        Comp::Eq => (),
                        Comp::Ordered => return Comp::Ordered,
                        Comp::NotOrdered => return Comp::NotOrdered,
                    }
                    idx += 1
                }
                if self_list.len() == other_list.len() {
                    return Comp::Eq;
                }
                if idx >= self_list.len() {
                    return Comp::Ordered;
                }
                Comp::NotOrdered
            }
        }
    }
}
pub fn parse_packet(input: &str) -> Vec<PacketMember> {
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
    stack.pop().expect("No stack to return!")
}

const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

fn display_packet(packet: &Vec<PacketMember>) {
    let mut strs = Vec::new();
    for el in packet {
        strs.push(format!("{}", el));
    }
    let str = strs.join(",");
    println!("[{}]", str);
}
#[cfg(test)]
#[test]
fn sort() {
    let pairs = TEST_INPUT.split("\n\n");
    let mut packets = Vec::new();
    for pair in pairs {
        let (packet_1, packet_2) = pair.split_once("\n").unwrap();
        let parsed_1 = parse_packet(packet_1);
        let parsed_2 = parse_packet(packet_2);
        packets.push(parsed_1);
        packets.push(parsed_2);
    }
    //insert markers
    packets.push(PacketMember::create_marker_2());
    packets.push(PacketMember::create_marker_6());
    println!("UNSORTED: --------");
    for p in &packets {
        display_packet(p);
    }
    println!("SORTED: --------");
    let sorted = sort_packets(packets);

    for pack in sorted {
        display_packet(&pack);
    }
    panic!();
}
pub fn sort_packets(mut packets: Vec<Vec<PacketMember>>) -> Vec<Vec<PacketMember>> {
    let mut swap = true;
    while swap {
        let mut x = 0;
        let mut y = 1;
        swap = false;
        while y < packets.len() {
            match compare_packet_pairs(&(packets[x].clone(), packets[y].clone())) {
                Comp::NotOrdered => {
                    let temp_y = packets[y].clone();
                    packets[y] = packets[x].clone();
                    packets[x] = temp_y;
                    swap = true;
                }
                _ => (),
            }
            x += 1;
            y += 1;
        }
    }
    packets
}
// #[test]
// fn sorting() {
//     let pairs = TEST_INPUT.split("\n\n");
//     let mut packets = Vec::new();

//     for pair in pairs {
//         let (packet_1_str, packet_2_str) = pair.split_once("\n").unwrap();
//         let packet_1 = parse_packet(packet_1_str);
//         let packet_2 = parse_packet(packet_2_str);
//         packets.push(packet_1);
//         packets.push(packet_2);
//     }
//     let sorted = sort_packets(packets);
// }
