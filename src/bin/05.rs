use itertools::Itertools;
use std::collections::HashMap;
use rayon::prelude::*;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rulesstr, orderingsstr) = input.trim().split("\n\n").collect_tuple().unwrap();
    let rules = rulesstr
        .trim()
        .lines()
        .map(|s| {
            s.trim()
                .split("|")
                .map(|s2| s2.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let orderings = orderingsstr
        .trim()
        .lines()
        .map(|s| {
            s.trim()
                .split(",")
                .map(|s2| s2.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (rules, orderings)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, orderings) = parse_input(input);
    let mut sum = 0;
    for ordering in orderings {
        let mut page_index = HashMap::new();
        for (ind, &pg) in ordering.iter().enumerate() {
            page_index.insert(pg, ind);
        }
        let mut ok = true;
        for rule in rules.iter() {
            let (r0, r1) = rule;
            if page_index.contains_key(r0) && page_index.contains_key(r1) {
                if page_index[r0] > page_index[r1] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            let pages = ordering.len();
            assert!(pages % 2 == 1); // has to be odd to get middle page
            let middle_page = ordering[(pages - 1) / 2]; // if 3 pages need index 1, if 5 pages need index 2, if 7 pages need index 3...
            sum += middle_page;
        }
    }

    return Some(sum);
}

fn check_ordering(rules: &Vec<(u32, u32)>, ordering: &Vec<u32>) -> bool {
    let mut page_index = HashMap::new();
    for (ind, &pg) in ordering.iter().enumerate() {
        page_index.insert(pg, ind);
    }
    for rule in rules.iter() {
        let (r0, r1) = rule;
        if page_index.contains_key(r0) && page_index.contains_key(r1) {
            if page_index[r0] > page_index[r1] {
                return false;
            }
        }
    }
    true
}


pub fn part_two(input: &str) -> Option<u32> {
    let (rules, orderings) = parse_input(input);

    let sum = orderings.par_iter().map(|ordering| {
        let ok = check_ordering(&rules, &ordering);
        if ok { 0 }
        else {
            let mut ord = ordering.clone();
            while (!check_ordering(&rules, &ord)) {
                for i in 0..ord.len() {
                    for j in (i+1)..ord.len() {
                        let left_pg = ord[i];
                        let right_pg = ord[j];
                        for (r0,r1) in rules.iter() {
                            if (*r1 == left_pg && *r0 == right_pg) { // wrong order, maybe I'll just swap them?
                                ord[j] = left_pg;
                                ord[i] = right_pg;
                                continue; // could probably keep going, but I think we should start over
                            }
                        }
                    }
                }
            }

            let pages = ord.len();
            assert!(pages % 2 == 1); // has to be odd to get middle page
            let middle_page = ord[(pages - 1) / 2]; // if 3 pages need index 1, if 5 pages need index 2, if 7 pages need index 3...
            //println!("Middle_page: {:?}",middle_page);
            return middle_page as u32;
        }
    }).sum();

    return Some(sum);
}

pub fn part_two_brute_force_too_slow(input: &str) -> Option<u32> {
    let (rules, orderings) = parse_input(input);
    //let mut sum = 0;

    let sum = orderings.par_iter().map(|ordering| {
        //for ordering in orderings.iter().enumerate() {
        let ok = check_ordering(&rules, &ordering);
        if ok { 0 }
        else {
            // we only care about ones that don't work at first
            let okord = ordering
                .iter()
                .permutations(ordering.len())
                .map(|p| p.iter().map(|v| **v).collect_vec())
                .find_or_last(|ord| {
                    check_ordering(&rules, ord)
                })
                .unwrap();
            let pages = okord.len();
            assert!(pages % 2 == 1); // has to be odd to get middle page
            let middle_page = okord[(pages - 1) / 2]; // if 3 pages need index 1, if 5 pages need index 2, if 7 pages need index 3...
            //sum += middle_page;
            println!("Middle_page: {:?}",middle_page);
            return middle_page as u32;
        }
    }).sum();

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
