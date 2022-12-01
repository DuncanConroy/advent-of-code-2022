use std::io::{BufRead, Read};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut elves = extract_elves_from_input(input);
    let top = find_calories_of_elf_with_most_calories(&mut elves);
    let top_three = find_calories_of_top_three_elves(&mut elves);

    println!("The elf with the most calories has {top} calories");
    println!("The top three elves carry {top_three} calories");
}

fn find_calories_of_top_three_elves(elves:&mut Vec<Elf>)->u32{
    elves.sort_by(|a, b| b.calories().cmp(&a.calories()));
    elves[0].calories() + elves[1].calories() + elves[2].calories()
}

fn find_calories_of_elf_with_most_calories(elves: &mut Vec<Elf>) -> u32 {
    elves.sort_by(|a, b| a.calories().cmp(&b.calories()));
    elves.last().unwrap().calories()
}

fn extract_elves_from_input(input: String) -> Vec<Elf> {
    input.split("\n\n").map(|elf_lines| {
        Elf { lines: elf_lines.lines().map(|it| it.to_string()).collect() }
    }).collect()
}

struct Elf {
    lines: Vec<String>,
}

impl Elf {
    fn calories(&self) -> u32 {
        self.lines.iter()
            .map(|it|it.trim())
            .filter(|it|
            !it.is_empty()).map(|it| it.parse::<u32>().expect(&format!("Couldn't parse {it}"))).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use crate::{Elf, extract_elves_from_input, find_calories_of_elf_with_most_calories, find_calories_of_top_three_elves};

    #[test]
    fn extracts_elves() {
        // given: prepared input
        let input = r#"123

        1234
        1423

        142445
        345
        "#;

        // when: extracting elves
        let result = extract_elves_from_input(input.into());

        // then: 3 elves are extracted
        assert_eq!(3, result.len());
    }

    #[test]
    fn sums_elves_calories() {
        // given: prepared Elf
        let elf = Elf {
            lines: vec!["100".into(), "200".into(), "300".into(), "".into()],
        };

        // when: calories are calculated
        let result = elf.calories();

        // then: correct calories are returned
        assert_eq!(600, result);
    }

    #[test]
    fn finds_elf_with_most_calories() {
        // given: prepared input
        let mut elves = vec![Elf {
            lines: vec!["100".into(), "200".into(), "300".into(), "".into()],
        }];

        // when: searching for elf with most calories
        let result = find_calories_of_elf_with_most_calories(&mut elves);

        // then: 3 elves are extracted
        assert_eq!(600, result);
    }

    #[test]
    fn finds_top_three_elves() {
        // given: prepared input
        let mut elves = vec![
            Elf {
                lines: vec!["100".into(), "200".into(), "300".into(), "".into()],
            },
            Elf {
                lines: vec!["120".into(), "200".into(), "300".into(), "".into()], // top 1
            },
            Elf {
                lines: vec!["104".into(), "200".into(), "300".into(), "".into()], // top 3
            },
            Elf {
                lines: vec!["100".into(), "200".into(), "300".into(), "".into()],
            },
            Elf {
                lines: vec!["110".into(), "200".into(), "300".into(), "".into()], // top 2
            },];

        // when: searching for elf with most calories
        let result = find_calories_of_top_three_elves(&mut elves);

        // then: 3 elves are extracted
        assert_eq!(620+610+604, result);
    }
}
