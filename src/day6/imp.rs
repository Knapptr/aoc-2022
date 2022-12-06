use std::collections::HashMap;
// abjajnc

pub fn find_packet_marker(input: &str, unique_length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut seen_chars = HashMap::new();

    let mut start_char_idx: usize = 0;
    let mut end_char_idx: usize = 0;

    while end_char_idx - start_char_idx < unique_length {
        let check_char = chars.get(end_char_idx).expect("no char at index");
        let check_char = seen_chars.insert(*check_char, end_char_idx);
        match check_char {
            Some(old_index) => {
                if old_index >= start_char_idx {
                    start_char_idx = old_index + 1;
                }
                end_char_idx += 1;
            }
            None => end_char_idx += 1,
        }
    }
    end_char_idx
}

#[cfg(test)]
#[test]
fn gets_packet_marker_case_1() {
    let test_str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(find_packet_marker(test_str, 4), 5);
}
#[test]
fn gets_packet_marker_case_2() {
    let test_str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(find_packet_marker(test_str, 4), 10);
}
