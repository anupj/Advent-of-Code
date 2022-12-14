#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, ch)| (ch, index + 1))
        // so that index starts at 1 instead of 0
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|sack| {
            let sack_compartment_len = sack.len() / 2;
            let compartment_a = &sack[0..sack_compartment_len];
            let compartment_b = &sack[sack_compartment_len..(sack_compartment_len * 2)];

            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, ch)| (ch, index + 1))
        .collect::<HashMap<char, usize>>();
    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[sack_a, sack_b, sack_c]| {
            let common_char = sack_a
                .chars()
                .find(|sack_a_char| sack_b.contains(*sack_a_char) && sack_c.contains(*sack_a_char))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
