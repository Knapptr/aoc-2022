use super::imp::find_packet_marker;

pub fn solve(input: &str) {
    println!("Part 1: Packet starts at: {}", find_packet_marker(input, 4));
    println!(
        "Part 2: Message starts at: {}",
        find_packet_marker(input, 14)
    );
}
