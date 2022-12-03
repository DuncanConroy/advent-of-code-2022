use std::ops::Index;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let score = calculate_sum_of_errornous_items(input.clone());
    println!("The total sum of errornous items is {score}");

    let score_round_two = calculate_sum_of_sticker_values(input);
    println!("The total sum of sticker values is {score_round_two}");
}

fn calculate_sum_of_sticker_values(input: String) -> i32 {
    create_groups_of_n(input, 3).iter()
        .map(|it| find_common_item(it))
        .map(|it| calculate_value(it.chars().next().unwrap()))
        .sum()
}

fn create_groups_of_n(input: String, group_size: usize) -> Vec<Vec<String>> {
    let mut lines = input.lines().into_iter().map(|it| it.trim());
    let mut groups: Vec<Vec<String>> = vec![vec![]];
    while let Some(line) = lines.next() {
        if groups.last().unwrap().len() == group_size {
            groups.push(Vec::new());
        }
        groups.last_mut().unwrap().push(line.to_string());
    }
    groups
}

fn find_common_item(input: &Vec<String>) -> String {
    input.first().unwrap().chars().find(|c| {
        input.iter().all(|line| line.contains(*c))
    }).unwrap().to_string()
}

fn calculate_sum_of_errornous_items(input: String) -> i32 {
    create_groups_of_n(input, 1).iter()
        .map(|it| find_common_item(&get_compartments(it.first().unwrap().to_string())))
        .map(|it| calculate_value(it.chars().next().unwrap()))
        .sum()
}

const ITEM_LIST: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn calculate_value(a: char) -> i32 {
    ITEM_LIST.iter().position(|&r| r == a).unwrap() as i32 + 1
}

fn get_compartments(line: String) -> Vec<String> {
    let half_line = line.len() >> 1;
    vec![line[..half_line].to_string(), line[half_line..].to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_correct_value_for_item_type() {
        // given: a list of items and expected values
        let items = vec![
            ('a', 1),
            ('b', 2),
            ('c', 3),
            ('d', 4),
            ('e', 5),
            ('f', 6),
            ('g', 7),
            ('h', 8),
            ('i', 9),
            ('j', 10),
            ('k', 11),
            ('l', 12),
            ('A', 27),
            ('B', 28),
            ('C', 29),
            ('D', 30),
            ('E', 31),
            ('F', 32),
            ('G', 33),
            ('H', 34),
            ('I', 35),
            ('J', 36),
            ('K', 37),
            ('L', 38),
        ];

        // when: we calculate the value for each item
        let result = items.iter().map(|(item, _)| calculate_value(*item)).collect::<Vec<i32>>();
        // then: we get the expected value
        assert_eq!(result, items.iter().map(|(_, value)| *value).collect::<Vec<i32>>());
    }

    #[test]
    fn creates_two_compartments_of_same_size() {
        // given: a rucksack
        let input = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", "vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrVvPwwTWBwg", "PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJQctTZtZT", "ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", "CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];

        // when: get_compartments is invoked
        let result = input.iter().map(|line| get_compartments(line.0.to_string())).collect::<Vec<Vec<String>>>();

        // then: two compartments are returned
        assert_eq!(result, input.iter().map(|line| vec![line.1.to_string(), line.2.to_string()]).collect::<Vec<Vec<String>>>());
    }

    #[test]
    fn finds_errornous_item_in_rucksack() {
        // given: a rucksack
        let input = get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());

        // when: we invoke the function to find the errornous item
        let result = find_common_item(&input);

        // then: we get the errornous item
        assert_eq!(result, "p".to_string());
    }

    #[test]
    fn calculates_sum_of_errornous_items() {
        // given: multiple rucksäcke
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#.to_string();

        // when: we process the input
        let result = calculate_sum_of_errornous_items(input);

        // then: we get the correct sum
        assert_eq!(result, 157);
    }

    #[test]
    fn creates_groups_of_three() {
        // given: a list of rucksäcke
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#.to_string();

        // when: we group the rucksäcke into groups of three
        let result = create_groups_of_n(input, 3);

        // then: we get two groups
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn finds_common_item_in_group() {
        // given: a group of three rucksäcke
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        // when: we search the common item
        let result = find_common_item(&input);

        // then: we get the common item
        assert_eq!(result, "r".to_string());
    }

    #[test]
    fn calculates_sum_of_sticker_values() {
        // given: a list of rucksäcke
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#.to_string();

        // when: we calculate the sum of the sticker values
        let result = calculate_sum_of_sticker_values(input);

        // then: we get the correct sum
        assert_eq!(result, 70);
    }
}
