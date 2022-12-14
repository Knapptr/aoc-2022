use crate::day13::imp::{sort_packets, PacketMember};

use super::imp::{compare_packet_pairs, parse_packet, Comp};

pub fn solve(input: &str) {
    let pairs = input.split("\n\n");

    let mut packets = Vec::new();
    // let mut index_list = Vec::new();
    for (idx, pair) in pairs.enumerate() {
        let (packet_1_str, packet_2_str) = pair.split_once("\n").unwrap();
        let packet_1 = parse_packet(packet_1_str);
        let packet_2 = parse_packet(packet_2_str);
        packets.push(packet_1);
        packets.push(packet_2);
        // match compare_packet_pairs((packet_1, packet_2)) {
        //     Comp::Ordered => index_list.push(idx + 1),
        //     Comp::Eq => {
        //         println!("Equality. Something might be off");
        //     }
        //     _ => (),
        // }
    }
    // println!("Part 1: {}", index_list.iter().sum::<usize>());
    // add locator packets;
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));
    let mut indicies = Vec::new();
    let sorted = sort_packets(packets);
    for (idx, packet) in sorted.into_iter().enumerate() {
        match compare_packet_pairs(&(packet.clone(), PacketMember::create_marker_2())) {
            Comp::Eq => {
                indicies.push(idx + 1);
                continue;
            }
            _ => (),
        }
        match compare_packet_pairs(&(packet, PacketMember::create_marker_6())) {
            Comp::Eq => {
                indicies.push(idx + 1);
                continue;
            }
            _ => (),
        }
    }
    println!("Indicies: {:?}", indicies);
    println!("Product: {:?}", indicies.iter().product::<usize>());
}
