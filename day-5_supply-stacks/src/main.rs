use std::collections::LinkedList;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let stacks = restack_crates_and_return_top_crate(input.clone(), false);
    println!("[Crate Mover 9000] The top crates are: {stacks:?}");

    let stacks = restack_crates_and_return_top_crate(input, true);
    println!("[Crate Mover 9001] The top crates are: {stacks:?}");
}

fn restack_crates_and_return_top_crate(input: String, runs_on_crate_mover_9001:bool) -> Vec<char> {
    let (stacks, instructions) = get_stacks_and_instructions(input);
    let mut stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);
    let sorted_stacks = execute_instructions(stacks, instructions, runs_on_crate_mover_9001);

    sorted_stacks.iter().map(|it| it.back().unwrap().clone()).collect()
}

fn get_stacks_and_instructions(input: String) -> (String, String) {
    input.split("\n\n").partition(|it| !it.contains("move"))
}

fn parse_stacks(input: String) -> Vec<LinkedList<char>> {
    let lines = input.lines();
    let mut stacks = vec![];
    lines.into_iter().rev().enumerate().for_each(|(index, it)| {
        println!("{index}: {it}");
        if index == 0 {
            let num_stacks = it.trim().chars().last().unwrap().to_string().parse::<usize>().unwrap();
            for _ in 0..num_stacks {
                stacks.push(LinkedList::new());
            }
        } else {
            it.bytes().collect::<Vec<u8>>().chunks(4)
                .enumerate().for_each(|(index, it)| {
                let letter = it[1] as char;
                if letter != ' ' {
                    stacks[index].push_back(letter);
                }
            });
        }
    });

    stacks
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    input.lines().into_iter().map(Instruction::from).collect()
}

fn execute_instructions(stacks: Vec<LinkedList<char>>, instructions: Vec<Instruction>, runs_on_crate_mover_9001: bool) -> Vec<LinkedList<char>> {
    let mut stacks = stacks;
    instructions.into_iter().for_each(|it| {
        let mut tmp_stack = LinkedList::new();
        let from = it.from - 1;
        let to = it.to - 1;
        for i in 0..it.iterations {
            let letter = stacks[from].pop_back().unwrap();
            if runs_on_crate_mover_9001 {
                tmp_stack.push_front(letter);
            } else {
                stacks[to].push_back(letter);
            }
        }
        if runs_on_crate_mover_9001 {
            stacks[to].append(&mut tmp_stack);
        }
    });

    stacks
}

struct Instruction {
    pub iterations: usize,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split(" ").enumerate();
        let mut parts = parts.skip(1);
        let iterations = parts.next().unwrap().1.parse::<usize>().unwrap();
        let mut parts = parts.skip(1);
        let from = parts.next().unwrap().1.parse::<usize>().unwrap();
        let mut parts = parts.skip(1);
        let to = parts.next().unwrap().1.parse::<usize>().unwrap();
        Instruction { iterations, from, to }
    }
}


#[cfg(test)]
mod tests {
    use std::{assert_eq, println, vec};
    use std::collections::LinkedList;
    use super::*;

    #[test]
    fn prefilters_input() {
        // given: crates and instructions
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#.to_string();

        // when: get_stacks_and_instructions is invoked
        let result = get_stacks_and_instructions(input);

        // then: we get two outputs: stacks and instructions
        assert!(result.0.contains("    [D]"));
        assert!(result.0.contains("[N] [C]"));
        assert!(result.1.contains("move 3 from 1 to 3"));
    }

    #[test]
    fn parses_stacks_from_input() {
        // given: crate stacks as input
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"#.to_string();

        // when: the parse function is invoked
        let mut result = parse_stacks(input);
        println!("{:?}", result);
        // then: the stacks are parsed correctly
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0].front().unwrap(), &'Z');
        assert_eq!(result[0].back().unwrap(), &'N');
        assert_eq!(result[1].len(), 3);
        assert_eq!(result[1].pop_front().unwrap(), 'M');
        assert_eq!(result[1].pop_front().unwrap(), 'C');
        assert_eq!(result[1].pop_front().unwrap(), 'D');
        assert_eq!(result[2].len(), 1);
        assert_eq!(result[2].front().unwrap(), &'P');
    }

    #[test]
    fn parses_instructions() {
        // given: instructions as input
        let input = r#"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#.to_string();

        // when: parse_instructions is invoked
        let result = parse_instructions(input);

        // then: we get the instructions parsed correctly
        assert_eq!(result[0].iterations, 1);
        assert_eq!(result[0].from, 2);
        assert_eq!(result[0].to, 1);
        assert_eq!(result[1].iterations, 3);
        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, 3);
        assert_eq!(result[2].iterations, 2);
        assert_eq!(result[2].from, 2);
        assert_eq!(result[2].to, 1);
        assert_eq!(result[3].iterations, 1);
        assert_eq!(result[3].from, 1);
        assert_eq!(result[3].to, 2);
    }

    #[test]
    fn executes_instruction_on_stacks() {
        // given: stacks and instructions
        let mut stacks = vec![
            LinkedList::from(['Z', 'N']),
            LinkedList::from(['M', 'C', 'D']),
            LinkedList::from(['P']),
        ];
        let instructions = vec![
            Instruction { iterations: 1, from: 2, to: 1 },//znd,mc,p
            Instruction { iterations: 3, from: 1, to: 3 },//,mc,pdnz
        ];

        // when: execute_instruction is invoked
        let mut result = execute_instructions(stacks, instructions, false);

        // then: the crates are stacked correctly
        assert_eq!(result[0].len(), 0);
        assert_eq!(result[1].len(), 2);
        assert_eq!(result[1].pop_front().unwrap(), 'M');
        assert_eq!(result[1].pop_front().unwrap(), 'C');
        assert_eq!(result[2].len(), 4);
        assert_eq!(result[2].pop_front().unwrap(), 'P');
        assert_eq!(result[2].pop_front().unwrap(), 'D');
        assert_eq!(result[2].pop_front().unwrap(), 'N');
        assert_eq!(result[2].pop_front().unwrap(), 'Z');
    }

    #[test]
    fn restacks_crates_from_input() {
        // given: crates and instructions
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#.to_string();

        // when: restack_crates is invoked
        let result = restack_crates_and_return_top_crate(input);

        // then: the crates are restacked correctly
        assert_eq!(result[0], 'C');
        assert_eq!(result[1], 'M');
        assert_eq!(result[2], 'Z');
    }

    #[test]
    fn runs_on_crate_mover_9001() {
        // given: instructions for crate mover 9001
        let mut stacks = vec![
            LinkedList::from(['Z', 'N']),
            LinkedList::from(['M', 'C', 'D']),
            LinkedList::from(['P']),
        ];
        let instructions = vec![
            Instruction { iterations: 1, from: 2, to: 1 },//znd,mc,p
            Instruction { iterations: 3, from: 1, to: 3 },//,mc,pznd
        ];

        // when: execute_instruction is invoked
        let mut result = execute_instructions(stacks, instructions, true);

        // then: the crates are stacked correctly
        assert_eq!(result[0].len(), 0);
        assert_eq!(result[1].len(), 2);
        assert_eq!(result[1].pop_front().unwrap(), 'M');
        assert_eq!(result[1].pop_front().unwrap(), 'C');
        assert_eq!(result[2].len(), 4);
        assert_eq!(result[2].pop_front().unwrap(), 'P');
        assert_eq!(result[2].pop_front().unwrap(), 'Z');
        assert_eq!(result[2].pop_front().unwrap(), 'N');
        assert_eq!(result[2].pop_front().unwrap(), 'D');
    }
}
