use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let score = get_total_number_of_overlapping_ranges(input.clone(), true);
    println!("The total sum of fully overlapping sections is {score}");

    let score_round_two = get_total_number_of_overlapping_ranges(input, false);
    println!("The total sum of partially overlapping sections is {score_round_two}");
}

fn get_total_number_of_overlapping_ranges(input: String, full: bool) -> usize {
    input.lines().into_iter().map(|it| it.trim())
        .map(|it| get_pair_of_sections(it.to_string()))
        .filter(|it| if full { fully_contained_in_ranges(&it[0], &it[1]) } else { check_overlap(&it[0], &it[1]) })
        .count()
}

fn get_pair_of_sections(input: String) -> Vec<Range<usize>> {
    let mut sections = input.split(",");
    sections.map(|it| {
        let numbers = it.split("-")
            .map(|b| b.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        numbers[0]..numbers[1]
    }).collect()
}

fn fully_contained_in_ranges(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.end <= b.end && a.start >= b.start ||
        b.end <= a.end && b.start >= a.start
}

fn check_overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.start && a.end >= b.start || b.start <= a.start && b.end >= a.start
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};
    use super::*;

    #[test]
    fn gets_pairs_of_sections() {
        // given: a string of section groups
        let input = "2-4,6-8".to_string();

        // when: we split the string into pairs of sections
        let result = get_pair_of_sections(input);

        // then: we get the correct pairs of sections
        assert_eq!(result, vec![2..4, 6..8]);
    }

    #[test]
    fn checks_if_range_is_fully_contained_by_the_other_range() {
        // given: two ranges
        let ranges = vec![2..8, 3..7];

        // when: fully_contained_in_ranges is called
        let result = fully_contained_in_ranges(&ranges[0], &ranges[1]);

        // then: we get the correct result
        assert_eq!(result, true);
    }

    #[test]
    fn gets_total_number_of_fully_contained_ranges() {
        // given: a string of section groups
        let input = r#"2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8"#.to_string();

        // when: get_total_number_of_fully_contained_ranges is called
        let result = get_total_number_of_overlapping_ranges(input, true);

        // then: we get the correct pairs of sections
        assert_eq!(result, 2);
    }

    #[test]
    fn checks_if_ranges_overlap() {
        // given: two ranges
        let ranges = vec![5..7, 7..9];

        // when: we check if they overlap
        let result = check_overlap(&ranges[0], &ranges[1]);

        // then: we get the correct result
        assert_eq!(result, true);
    }

    #[test]
    fn checks_if_ranges_do_not_overlap() {
        // given: two ranges
        let ranges = vec![2..4, 6..8];

        // when: we check if they overlap
        let result = check_overlap(&ranges[0], &ranges[1]);

        // then: we get the correct result
        assert_eq!(result, false);
    }

    #[test]
    fn gets_total_number_of_overlapping_ranges() {
        // given: a string of section groups
        let input = r#"2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
            9-9,4-6"#.to_string();

        // when: get_total_number_of_overlapping_ranges is called
        let result = get_total_number_of_overlapping_ranges(input, false);

        // then: we get the correct pairs of sections
        assert_eq!(result, 4);
    }
}
