use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day_01.txt").unwrap();

    let mut elves: Vec<u32> = file
        .split("\n\n")
        .map(|l| l.lines().map(|e| e.parse::<u32>().unwrap()).sum())
        .collect();

    let max: &u32 = elves.iter().max().unwrap();
    println!("Part one: {}", max);

    elves.sort_by(|a, b| b.cmp(a));
    println!("Part two: {}", elves[0] + elves[1] + elves[2]);
}
