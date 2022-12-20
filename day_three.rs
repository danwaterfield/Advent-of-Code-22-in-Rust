use itertools::Itertools;
fn main() -> () {
    let output_one: u32 = part_one();
    print!("The answer to day three, part one is {}", output_one);

    let output_two: u32 = part_two();
    println!("The answer to the second part of day 3 is: {}", output_two);
}


fn part_one() -> u32 {
    let input = include_str!("../../input/AO3_input.txt");
    input.lines().filter_map(|line| {
        let line = line.as_bytes();
        let (left, right) = line.split_at(line.len() / 2);

        left
        .iter()
        .find(|item| right.contains(item))
        .map(|item| match item {
            b'a'..=b'z' => (item - b'a')+ 1,
            _ =>(item - b'A') + 1 + 26,
        }
        as u32)
    })
    .sum()
}

fn part_two() -> u32 {
    let input = include_str!("../../input/AO3_input.txt");
    input.lines()
    .map(|line| line.as_bytes())
    .tuples()
    .filter_map(|(sack1, sack2, sack3)| {
        sack1 
        .iter()
        .find(|item| sack2.contains(item) && sack3.contains(item))
        .map(|item| match item {
            b'a'..=b'z' => (item - b'a') + 1,
            _ => (item - b'A') + 1 + 26,

        } as u32
    )
    })
    .sum()
}
