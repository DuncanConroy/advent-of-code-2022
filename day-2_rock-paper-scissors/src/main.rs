use crate::Symbol::{Paper, Rock, Scissors};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let score: i32 = input.lines().map(|line| {
        let split = line.split(" ").map(map_aliases).collect::<Vec<Symbol>>();
        calculate_score(split[0], split[1])
    }).sum();
    println!("The score is {score}");

    let score: i32 = input.lines().map(|line| {
        let split = line.split(" ").map(map_aliases).collect::<Vec<Symbol>>();
        calculate_score(split[0], map_symbol_for_outcome(split[0], split[1]))
    }).sum();
    println!("The score is {score}");
}

fn calculate_score(a: Symbol, b: Symbol) -> i32 {
    calculate_reward(&a, &b) + b.get_value()
}

fn calculate_reward(a: &Symbol, b: &Symbol) -> i32 {
    if a == b {
        return 3;
    } else if &a.looses_against() == b {
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

impl Symbol {
    fn from_str(s: &str) -> Symbol {
        let s = s.to_uppercase();
        match s.as_str() {
            "A" => Rock,
            "X" => Rock,
            "B" => Paper,
            "Y" => Paper,
            "C" => Scissors,
            "Z" => Scissors,
            _ => panic!("Unknown symbol"),
        }
    }

    fn wins_against(&self) -> Symbol {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn looses_against(&self) -> Symbol {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn map_aliases(alias: &str) -> Symbol {
    Symbol::from_str(alias)
}

fn map_symbol_for_outcome(a: Symbol, b: Symbol) -> Symbol {
    match b {
        Rock => a.wins_against(),
        Paper => a,
        Scissors => a.looses_against(),
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
        let result = rock.wins_against();

        // then: rock wins
        assert_eq!(scissors, result);
    }

    #[test]
    fn paper_wins_against_rock() {
        // given: paper and rock
        let paper = Paper;
        let rock = Rock;

        // when: paper is compared to rock
        let result = paper.wins_against();

        // then: paper wins
        assert_eq!(rock, result);
    }

    #[test]
    fn scissors_wins_against_paper() {
        // given: scissors and paper
        let scissors = Scissors;
        let paper = Paper;

        // when: scissors is compared to paper
        let result = scissors.wins_against();

        // then: scissors wins
        assert_eq!(paper, result);
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

    #[test]
    fn finds_correct_symbol_for_outcome() {
        // given: list of symbol pairs
        let invocations_expectations = vec![
            (Rock, Rock, Scissors),
            (Rock, Paper, Rock),
            (Rock, Scissors, Paper),
            (Paper, Rock, Rock),
            (Paper, Paper, Paper),
            (Paper, Scissors, Scissors),
            (Scissors, Rock, Paper),
            (Scissors, Paper, Scissors),
            (Scissors, Scissors, Rock),
        ];

        // when: map symbol for outcome is invoked
        let output: Vec<Symbol> = invocations_expectations.iter().map(|it| map_symbol_for_outcome(it.0, it.1)).collect();
        // then: correct symbol is returned
        let expected: Vec<Symbol> = invocations_expectations.iter().map(|it| it.2).collect();
        assert_eq!(expected, output);
    }
}
