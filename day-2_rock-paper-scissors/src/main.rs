use crate::Symbol::{Paper, Rock, Scissors};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let score: i32 = input.lines().map(|line| {
        let split = line.split(" ").map(map_aliases).collect::<Vec<Symbol>>();
        calculate_score(split[0], split[1])
    }).sum();
    println!("The score is {score}");
}

fn calculate_score(a: Symbol, b: Symbol) -> i32 {
    let outcome = compare(&a, &b);
    let result = match b {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    } + outcome;

    result
}

fn compare(a: &Symbol, b: &Symbol) -> i32 {
    if a == b {
        return 3;
    }
    if a == &Scissors && b == &Rock {
        return 6;
    }
    if a == &Paper && b == &Scissors {
        return 6;
    }
    if a == &Rock && b == &Paper {
        return 6;
    }
    0
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

const A: &str = "A";
const X: &str = "X";
const B: &str = "B";
const Y: &str = "Y";
const C: &str = "C";
const Z: &str = "Z";

fn map_aliases(alias: &str) -> Symbol {
    match alias.to_uppercase().as_str() {
        A => Rock,
        X => Rock,
        B => Paper,
        Y => Paper,
        C => Scissors,
        Z => Scissors,
        _ => panic!("Unknown alias {alias}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_wins_against_scissors() {
        // given: rock and scissors
        let rock = Rock;
        let scissors = Scissors;

        // when: rock is compared to scissors
        let result = compare(&rock, &scissors);

        // then: rock wins
        assert_eq!(1, result);
    }

    #[test]
    fn paper_wins_against_rock() {
        // given: paper and rock
        let paper = Paper;
        let rock = Rock;

        // when: paper is compared to rock
        let result = compare(&paper, &rock);

        // then: paper wins
        assert_eq!(1, result);
    }

    #[test]
    fn scissors_wins_against_paper() {
        // given: scissors and paper
        let scissors = Scissors;
        let paper = Paper;

        // when: scissors is compared to paper
        let result = compare(&scissors, &paper);

        // then: scissors wins
        assert_eq!(1, result);
    }

    #[test]
    fn maps_aliases_to_correct_symbol() {
        // given: aliases
        let input = vec!["a", "b", "c", "A", "B", "C", "x", "y", "z", "X", "Y", "Z"];

        // when: map_alias is invoked
        let output: Vec<Symbol> = input.iter().map(|it| map_aliases(it)).collect();

        // then: aliases are mapped correctly
        let expected = vec![Rock, Paper, Scissors, Rock, Paper, Scissors, Rock, Paper, Scissors, Rock, Paper, Scissors];
        assert_eq!(expected, output);
    }

    #[test]
    fn calculates_correct_score() {
        // given: tuples of invocation
        let invocations_expectations = vec![
            (Rock, Rock, 4),
            (Rock, Paper, 8),
            (Rock, Scissors, 3),
            (Paper, Rock, 1),
            (Paper, Paper, 5),
            (Paper, Scissors, 9),
            (Scissors, Rock, 7),
            (Scissors, Paper, 2),
            (Scissors, Scissors, 6),
        ];

        // when: calculate_score is invoked
        let output: Vec<i32> = invocations_expectations.iter().map(|it| calculate_score(it.0, it.1)).collect();

        // then: correct score is returned
        let expected: Vec<i32> = invocations_expectations.iter().map(|it| it.2).collect();
        assert_eq!(expected, output);
    }
}
