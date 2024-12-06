advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = tmp
        .iter()
        .map(|x| x.into_iter().collect_tuple().unwrap())
        .unzip();
    left.sort();
    right.sort();
    return Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let (left, mut right): (Vec<u32>, Vec<u32>) = tmp
        .iter()
        .map(|x| x.into_iter().collect_tuple().unwrap())
        .unzip();
    right.sort();
    Some(
        left.iter()
            .map(|l| right.iter().filter(|r| *l == **r).count() as u32 * l)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
