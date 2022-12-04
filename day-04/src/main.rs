fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut pairs: Vec<((u32, u32), (u32, u32))> = Vec::new();
    for line in INPUT.lines() {
        // TODO: clean up this mess
        let mut pair = line.split(',');
        let mut left = pair.next().unwrap().split('-');
        let left_start: u32 = left.next().unwrap().parse().unwrap();
        let left_end: u32 = left.next().unwrap().parse().unwrap();
        let mut right = pair.next().unwrap().split('-');
        let right_start: u32 = right.next().unwrap().parse().unwrap();
        let right_end: u32 = right.next().unwrap().parse().unwrap();
        pairs.push(((left_start, left_end), (right_start, right_end)));
    }

    let mut fully_contained = 0;
    for ((ls, le), (rs, re)) in &pairs {
        // is left contained in right
        if ls >= rs && le <= re {
            fully_contained += 1;
            continue;
        }
        // is right contained in left
        if rs >= ls && re <= le {
            fully_contained += 1;
            continue;
        }
    }
    println!("Part 1: {}", fully_contained);

    let mut partial_overlap = 0;
    for ((ls, le), (rs, re)) in pairs {
        let left = ls..=le;
        let right = rs..=re;
        for l in left {
            if right.contains(&l) {
                partial_overlap += 1;
                break;
            }
        }
    }
    println!("Part 2: {}", partial_overlap);
}
