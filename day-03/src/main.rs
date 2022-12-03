fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut priority_sum = 0;
    let mut backpacks = Vec::new();
    for line in INPUT.lines() {
        let items: Vec<char> = line.chars().collect();
        backpacks.push(items);
    }

    for backpack in &backpacks {
        let length = backpack.len();
        let (compartment1, compartment2) = backpack.split_at(length / 2);
        for item in compartment1 {
            if compartment2.contains(item) {
                priority_sum += priority(item);
                break;
            }
        }
    }
    println!("Part 1: {}", priority_sum);

    let mut groups = backpacks.chunks(3);
    let mut badge_sum = 0;
    while let Some([b1, b2, b3]) = groups.next() {
        for item in b1 {
            if b2.contains(item) && b3.contains(item) {
                badge_sum += priority(item);
                break;
            }
        }
    }
    println!("Part 2: {}", badge_sum);
}

fn priority(item: &char) -> usize {
    let lower: Vec<char> = ('a'..='z').collect();
    let upper: Vec<char> = ('A'..='Z').collect();
    let priorities = [lower, upper].concat();
    let position = priorities
        .iter()
        .position(|priority| priority == item)
        .unwrap();
    position + 1
}
