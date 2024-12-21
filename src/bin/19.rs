use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse_input(s: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = s.lines().collect_vec();
    let towels = lines[0].split(",").map(|s| s.trim()).collect_vec();
    let patterns = lines.iter().skip(2).map(|ln| ln.trim()).collect_vec();
    (towels, patterns)
}

fn can_match(towels_by_first_stripe: &HashMap<char, Vec<&str>>, pattern: &str) -> bool {
    let first_ch = pattern.chars().nth(0).unwrap();
    for towel in towels_by_first_stripe.get(&first_ch).unwrap_or(&vec![]) {
        if pattern.starts_with(towel) {
            if pattern.len() == towel.len() {
                return true;
            } else {
                if can_match(towels_by_first_stripe, &pattern[towel.len()..]) {
                    return true;
                } // if not, we try another
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, patterns) = parse_input(input);
    println!("Towels: {:?}", towels);
    println!("Patterns: {:?}", patterns);
    let towels_by_first_stripe = towels
        .iter()
        .map(|twl| (twl.chars().nth(0).unwrap(), *twl))
        .into_group_map();
    println!("{:?}", towels_by_first_stripe);
    let mut count = 0u32;
    for pattern in patterns {
        if can_match(&towels_by_first_stripe, pattern) {
            count += 1;
        }
    }
    Some(count)
}

fn ways_to_match<'a>(
    towels_by_first_stripe: &HashMap<char, Vec<&str>>,
    cache: &mut HashMap<&'a str, u64>,
    pattern: &'a str,
) -> u64 {
    let first_ch = pattern.chars().nth(0).unwrap();
    let mut ways = 0u64;
    //println!("  Pattern: {:?}", pattern);
    for towel in towels_by_first_stripe.get(&first_ch).unwrap_or(&vec![]) {
        if pattern.starts_with(towel) {
            if pattern.len() == towel.len() {
                ways += 1;
            } else {
                let pat = &pattern[towel.len()..];
                if cache.contains_key(pat) {
                    ways += cache.get(pat).unwrap();
                } else {
                    ways += ways_to_match(towels_by_first_stripe, cache, &pat);
                }
            }
        }
    }
    cache.insert(pattern.clone(), ways);
    ways
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, patterns) = parse_input(input);
    println!("Towels: {:?}", towels);
    println!("Patterns: {:?}", patterns);
    let towels_by_first_stripe = towels
        .iter()
        .map(|twl| (twl.chars().nth(0).unwrap(), *twl))
        .into_group_map();
    println!("{:?}", towels_by_first_stripe);
    let mut cache: HashMap<&str, u64> = HashMap::new();
    let mut count = 0u64;
    for pattern in patterns {
        println!("{:?}  ({})", pattern, count);
        count += ways_to_match(&towels_by_first_stripe, &mut cache, pattern);
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
