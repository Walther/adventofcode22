fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut left_column: Vec<u32> = Vec::new();
    let mut right_column: Vec<u32> = Vec::new();
    for line in INPUT.lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let left = splits[0];
        let right = splits[1];
        left_column.push(hand_to_value(left));
        right_column.push(hand_to_value(right));
    }
    let rounds = left_column.len();

    let mut naive_score = 0;
    (0..rounds).for_each(|i| {
        let opponent = left_column[i];
        let me = right_column[i];
        let score = me + match (me, opponent) {
            (1,2) | (2,3) | (3,1) => 0,
            (1,1) | (2,2) | (3,3) => 3,
            (1,3) | (2,1) | (3,2) => 6,
            _ => panic!("Unexpected round")
        };
        naive_score += score;
    });

    let mut correct_score = 0;
    (0..rounds).for_each(|i| {
        let opponent = left_column[i];
        let me = right_column[i];
        let score = match (me, opponent) {
            (1, 1) => 3, // lose to rock with scissors
            (1, 2) => 1, // lose to paper with rock
            (1, 3) => 2, // lose to scissors with paper
            (2, _) => 3 + opponent, // draw
            (3, 1) => 6 + 2, // win rock with paper
            (3, 2) => 6 + 3, // win paper with scissors
            (3, 3) => 6 + 1, // win scissors with rock
            _ => panic!("Unexpected round")
        };
        correct_score += score
    });

    println!("Part 1: {}", naive_score);
    println!("Part 2: {}", correct_score);
}

fn hand_to_value(hand: &str) -> u32 {
    match hand {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Unexpected hand")
    }
}
