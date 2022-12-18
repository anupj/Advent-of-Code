use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> String {
    let window_size = 4;

    let characters = input.chars().collect::<Vec<char>>();
    let sequence = characters
        .windows(window_size)
        .enumerate()
        .find(|(_i, slice)| {
            let unique_set_of_chars = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == unique_set_of_chars.len()
        })
        .unwrap();
    (sequence.0 + window_size).to_string()
}

pub fn process_part2(input: &str) -> String {
    let window_size = 14;

    let characters = input.chars().collect::<Vec<char>>();
    let sequence = characters
        .windows(window_size)
        .enumerate()
        .find(|(_i, slice)| {
            let unique_set_of_chars = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == unique_set_of_chars.len()
        })
        .unwrap();
    (sequence.0 + window_size).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    // const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
