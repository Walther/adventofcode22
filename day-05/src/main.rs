use std::{num::ParseIntError, str::FromStr};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut lines = INPUT.lines();
    // TODO: implement parsing for the crate input
    let crates: Vec<Vec<char>> = vec![
        vec![], // HACK: instructions are 1-indexed, add an empty zero pile
        vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'],
        vec!['D', 'Q', 'L'],
        vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'],
        vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'],
        vec!['V', 'G', 'L', 'F', 'Z', 'S'],
        vec!['D', 'G', 'N', 'P'],
        vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'],
        vec!['C', 'P', 'D', 'M', 'S'],
        vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'],
    ];

    for _ in 0..10 {
        lines.next();
    }
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        let instruction: Instruction = line.parse().unwrap();
        instructions.push(instruction);
    }

    let mut crates_p1 = crates.clone();
    for &Instruction { count, from, to } in &instructions {
        for _ in 0..count {
            let moving = crates_p1[from].pop().unwrap();
            crates_p1[to].push(moving);
        }
    }
    let mut top_string = String::new();
    (1..=9).for_each(|i| {
        top_string.push(*crates_p1[i].last().unwrap());
    });
    println!("Part 1: {}", top_string);

    let mut crates_p2 = crates;
    for Instruction { count, from, to } in instructions {
        let index = crates_p2[from].len() - count;
        let mut moving = crates_p2[from].split_off(index);
        crates_p2[to].append(&mut moving);
    }
    let mut top_string = String::new();
    (1..=9).for_each(|i| {
        top_string.push(*crates_p2[i].last().unwrap());
    });
    println!("Part 2: {}", top_string);
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let instruction = Instruction {
            count: parts[1].parse()?,
            from: parts[3].parse()?,
            to: parts[5].parse()?,
        };
        Ok(instruction)
    }
}
