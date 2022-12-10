use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let data: Vec<char> = INPUT.chars().collect();
    let first_packet = find_start_of_packet(&data);
    println!("Part 1: {}", first_packet);
    let first_message = find_start_of_message(&data);
    println!("Part 2: {}", first_message);
}

pub fn find_start_of_packet(data: &[char]) -> usize {
    find_start_of_x(data, 4)
}

pub fn find_start_of_message(data: &[char]) -> usize {
    find_start_of_x(data, 14)
}

fn find_start_of_x(data: &[char], window_size: usize) -> usize {
    let iter = data.windows(window_size);
    let mut index = window_size; // index for the end of window
    for slice in iter {
        let set: HashSet<&char> = slice.iter().collect();
        if set.len() == window_size {
            break;
        }
        index += 1;
    }

    index
}
