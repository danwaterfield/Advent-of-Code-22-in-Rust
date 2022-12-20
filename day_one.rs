use itertools::Itertools;
use std::io;

fn part_one() -> io::Result<()> {
    

    let lines = include_str!("input/AO1_input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let result = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        // 👇
        .max();
    println!("{result:?}");

    Ok(())
}

fn main() {
    let res = part_two();
    println!("{:?}", res);
}


fn part_two() -> u32 {
let input = std::fs::read_to_string("src/input/AO1_input.txt").unwrap();

input
    .split("\n\n")
    .map(|elf| {
        elf.lines()
            .filter_map(|s| s.parse::<u32>().ok())
            .sum::<u32>()
    })
    .sorted()
    .rev()
    .take(3)
    .sum::<u32>()
}