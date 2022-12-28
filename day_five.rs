use regex::Regex;
use std::collections::HashMap;




fn main() {
    let string = include_str!("../../input/AO5_input.txt");
    let mut split_iter = string.split("\n\n");

    let first_half: Vec<&str> = split_iter
        .next()
        .expect("There's no first half")
        .split("\n") // split on carriage return/line break
        .collect();
    let second_half: Vec<&str> = split_iter
        .next()
        .expect("no second half found")
        .split("\n")
        .collect();


     let stacks = get_stacks_hashmap(first_half);
     let part_one_solution = part_one(stacks.clone(), &second_half);
     println!("The solution to part one, day 5 is {part_one_solution}");

     let part_two_solution = second_half_solution(stacks.clone(), &second_half);
     println!("And the solution to part 2, day 5, is {part_two_solution}")
}


/* 
pub fn part_one(mut stacks: HashMap<char, String>, lines: &Vec<&str>) -> String {
    // some content
    for line in lines {
        let instructions = get_instructions(line);

        let stack_to_take_from_idx = &char::from_digit(instructions[1], 10).unwrap();
        let stack_to_take_from = stacks.get(stack_to_take_from_idx)
                .expect("There should be a string to move");

                let stack_to_add_to_idx = &char::from_digit(instructions[2], 10).unwrap();
                let stack_to_add_to = stacks
                    .get(stack_to_add_to_idx)
                    .expect("There's no string to move, sorry");

                    let split_idx = stack_to_take_from.len() - (instructions[0] as usize);
                    let first = &stack_to_add_to_idx[..split_idx];
                    let second = &stack_to_take_from[split_idx..];
                    let reversed = second.chars().rev().collect::<String>();

                    let new_stack = format!("{}{}", stack_to_add_to, reversed);

                    stacks.insert(*stack_to_take_from_idx, first.to_string());
                    stacks.insert(*stack_to_add_to_idx, new_stack);


    }
    let result = get_last_letters_of_stacks(stacks);
    result
}
*/
pub fn part_one(mut stacks: HashMap<char, String>, lines: &Vec<&str>) -> String {
    // some content
    for line in lines {
        let instructions = get_instructions(line);

        let stack_to_take_from_idx = char::from_digit(instructions[1], 10).unwrap();
        let stack_to_take_from = stacks.get(&stack_to_take_from_idx)
                .expect("There should be a string to move");

        let stack_to_add_to_idx = char::from_digit(instructions[2], 10).unwrap();
        let stack_to_add_to = stacks
            .get(&stack_to_add_to_idx)
            .expect("There's no string to move, sorry");

        // Calculate the correct split index here
        let split_idx = stack_to_take_from.len() - (instructions[0] as usize);
        let first = &stack_to_take_from[..split_idx];
        let second = &stack_to_take_from[split_idx..];
        let reversed = second.chars().rev().collect::<String>();

        let new_stack = format!("{}{}", stack_to_add_to, reversed);

        stacks.insert(stack_to_take_from_idx, first.to_string());
        stacks.insert(stack_to_add_to_idx, new_stack);
    }

    let result = get_last_letters_of_stacks(stacks);
    result
}



//get the instructions

fn get_instructions(line: &str) -> Vec<u32> {
    let reg = Regex::new(r"\d+").unwrap();
    return reg
    .find_iter(line)
    .map(|n| {
        n.as_str()
        .parse::<u32>()
        .expect("Didn't find a parsable number")
    })
    .collect();
}

fn get_last_letters_of_stacks(stacks: HashMap<char, String>) -> String {
    let mut result = String::from("");
    let stacks_size = stacks.keys().len();
    for idx in 1..stacks_size + 1 {
        let stack_idx = &char::from_digit(idx as u32, 10).unwrap();
        let last_letter = stacks
            .get(stack_idx)
            .expect("There should be a stack index :(")
            .chars()
            .last()
            .expect("There should be a character");
        result.push(last_letter);    
    }
    result
}

fn get_stacks_hashmap(lines: Vec<&str>) -> HashMap<char, String> {
    let mut stacks_map: HashMap<char, String> = HashMap::new();

    let last_line = lines.last().expect("Unable to get last element");
    let last_line_length = last_line.len();

    for line_idx in (1..last_line_length).step_by(4) {
        let stack_number = last_line.as_bytes()[line_idx] as char;

        for line_num in (0..(lines.len() - 1)).rev() {
            let character = lines[line_num].as_bytes()[line_idx] as char;
            if character.is_alphabetic(){
                stacks_map
                    .entry(stack_number)
                    .and_modify(|x| x.push(character))
                    .or_insert(character.to_string());
            }
            
        }
    }
    stacks_map

}


fn second_half_solution(mut stacks: HashMap<char, String>, lines: &Vec<&str>) -> String {
    for line in lines {
        let instructions = get_instructions(line);
        let stack_to_take_from_idx = &char::from_digit(instructions[1], 10)
        .unwrap();

        let stack_to_take_from = stacks
            .get(stack_to_take_from_idx)
            .expect("Expected a string to move in stack to take from");
        
        let stack_to_add_to_idx = &char::from_digit(instructions[2], 10)
            .unwrap();

        let stack_to_add_to = stacks
            .get(stack_to_add_to_idx)
            .expect("Expected a string to move in stack to add to");
        
        let split_idx = stack_to_take_from
            .len() - (instructions[0] as usize);

        let first_part = &stack_to_take_from[..split_idx];
        let second_part = &stack_to_take_from[split_idx..];
        let new_stack = format!("{}{}", stack_to_add_to, second_part);

        //don't mess up the order here

        stacks.insert(*stack_to_take_from_idx, first_part.to_string());
        stacks.insert(*stack_to_add_to_idx, new_stack);
    


    }
    let result = get_last_letters_of_stacks(stacks);

    result
}

