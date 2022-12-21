fn main() {
    let part_one = part_one();
    let part_two: usize = part_two();
    println!("The answer to part 1 is {}, and the answer to part two is {}", part_one, part_two);
}

const INPUT: &str = include_str!("../../input/AO4_input.txt");

fn ranges() -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
    INPUT.lines()
    .map(|line| {
        let mut split = line.split(|c| c == ',' || c == '-');
        let left_start = split.next().unwrap().parse::<usize>().unwrap();
        let left_end = split.next().unwrap().parse::<usize>().unwrap();
        let right_start = split.next().unwrap().parse::<usize>().unwrap();
        let right_end = split.next().unwrap().parse::<usize>().unwrap();
        ((left_start, left_end), (right_start, right_end))

    })
}


pub fn part_one() -> usize {
    ranges()
    .filter(|(left, right)| {
        (left.0 <= right.0 && left.1 >= right.0 && left.0 <= right.1 && left.1 >= right.1)
        || ( right.0 <= left.0 
            && right.1 >= left.0
        && right.0 <= left.1 
        && right.1 >= left.1)

    })
    .count()
}

pub fn part_two() -> usize {
    ranges()
        .filter(|(left, right)| {
            left.0 <= right.0 && left.1 >= right.0
                || left.0 <= right.1 && left.1 >= right.1
                || right.0 <= left.0 && right.1 >= left.0
                || right.0 <= left.1 && right.1 >= left.1
        })
        .count()
}