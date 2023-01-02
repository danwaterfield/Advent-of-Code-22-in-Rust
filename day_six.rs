use literal::{set, SetLiteral};
use std::{collections::HashSet, hash::Hash};


pub fn main() {
    let string = include_str!("../../input/AO6_input.txt");
    let bytes = string.as_bytes();
    let part_one_sol = get_part_one_solution(bytes);
    let part_two_sol = get_part_two_solution(string);
    println!("{part_one_sol}");
    println!("{part_two_sol}")
}


fn get_part_one_solution(bytes: &[u8]) -> i32 {
    let mut result = 0;
    for i in 0..bytes.len() - 4 {
        let set: HashSet<char> = 
        set! { bytes[i] as char, bytes[i+1] as char, bytes[i+2] as char, bytes[i+3] as char };
        if set.len() == 4 {
            result = i + 4;
            break
        }
    }
    return result as i32;
}

fn get_part_two_solution(string: &str) -> i32 {
    let mut result = 0;
    for i in 0..string.as_bytes().len() - 14 {
        let slice = &string[i..i +14];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());
        if set.len() == 14 {
            result = i + 14;
            break;
        }
    }
    return result as i32;
}