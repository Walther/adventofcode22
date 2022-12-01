fn main() {
    const INPUT: &str = include_str!("input.txt");
    // The input has newline-separated integers representing snacks
    // Each number represents the amount of calories in the snack
    // Two newlines separate the snacks into groups, each carried by an elf
    let elves = INPUT.split("\n\n");
    let mut calories = Vec::new();
    for elf in elves {
        let snacks: Vec<u32> = elf
            .lines()
            .map(|snack| snack.parse().expect("Unable to parse snack as u32"))
            .collect();
        let total_calories: u32 = snacks.iter().sum();
        calories.push(total_calories);
    }
    calories.sort();
    calories.reverse();
    let [top1, top2, top3] = [calories[0], calories[1], calories[2]];

    println!("Part 1: {}", top1);
    println!("Part 2: {}", top1 + top2 + top3);
}
