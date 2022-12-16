use crate::day13::imp::PacketMember;

pub fn solve(input: &str) {
    let pairs = input.split("\n\n");
    // let pairs = TEST_INPUT.split("\n\n");

    let mut packets = Vec::new();
    // let mut index_list = Vec::new();
    let mut correctly_ordered_pairs = Vec::new();
    for (idx, pair) in pairs.enumerate() {
        let (packet_1_str, packet_2_str) = pair.split_once("\n").unwrap();
        let packet_1 = PacketMember::create_from_input(packet_1_str);
        let packet_2 = PacketMember::create_from_input(packet_2_str);
        if packet_1 < packet_2 {
            correctly_ordered_pairs.push(idx + 1)
        }
        packets.push(packet_1);
        packets.push(packet_2);
    }

    // Part 1
    println!("Part 1: {}", correctly_ordered_pairs.iter().sum::<usize>());

    // Add Message Markers
    packets.push(PacketMember::create_marker_2());
    packets.push(PacketMember::create_marker_6());

    packets.sort();

    let mut indicies = Vec::new();
    for (idx, packet) in packets.into_iter().enumerate() {
        if packet == PacketMember::create_marker_2() || packet == PacketMember::create_marker_6() {
            indicies.push(idx + 1);
        }
    }

    // Part 2
    println!("Part 2: {:?}", indicies.iter().product::<usize>());
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
