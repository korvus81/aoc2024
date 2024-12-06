use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let min_max_chg = tmp
        .iter()
        .map(|levels| {
            (&levels[0..(levels.len() - 1)])
                .iter()
                .clone()
                .zip(&levels[1..(levels.len())])
                .map(|(first, second)| (*second - *first))
                .fold((i32::MAX, i32::MIN), |(min_chg, max_chg), chg| {
                    (std::cmp::min(min_chg, chg), std::cmp::max(max_chg, chg))
                })
        })
        .collect_vec();
    println!("{:?}", min_max_chg);
    let tmp2 = min_max_chg
        .iter()
        .map(|(min_chg, max_chg)| {
            let min_abs = min_chg.abs();
            let max_abs = max_chg.abs();

            if (min_abs >= 1 && min_abs <= 3)
                && (max_abs >= 1 && max_abs <= 3)
                && ((*min_chg < 0 && *max_chg < 0) || (*min_chg > 0 && *max_chg > 0))
            {
                1
            } else {
                0
            }
        })
        .collect_vec();
    println!("{:?}", tmp2);
    return Some(tmp2.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut safe_report_counter = 0;
    for base_levels in tmp.iter() {
        let mut possible_levels = (0..(base_levels.len() - 1))
            .map(|idx| {
                let (l1, l2) = base_levels.split_at(idx);
                [l1, &l2[1..]].concat()
            })
            .collect_vec();
        // I don't leave off the last one
        possible_levels.push(base_levels.clone()[0..(base_levels.len() - 1)].to_vec());
        possible_levels.push(base_levels.clone());
        println!("{:?}", possible_levels);
        if possible_levels.iter().any(|levels| {
            let (min_chg, max_chg) = (&levels[0..(levels.len() - 1)])
                .iter()
                .clone()
                .zip(&levels[1..(levels.len())])
                .map(|(first, second)| (*second - *first))
                .fold((i32::MAX, i32::MIN), |(min_chg, max_chg), chg| {
                    (std::cmp::min(min_chg, chg), std::cmp::max(max_chg, chg))
                });
            let min_abs = min_chg.abs();
            let max_abs = max_chg.abs();

            if (min_abs >= 1 && min_abs <= 3)
                && (max_abs >= 1 && max_abs <= 3)
                && ((min_chg < 0 && max_chg < 0) || (min_chg > 0 && max_chg > 0))
            {
                true
            } else {
                false
            }
        }) {
            safe_report_counter += 1;
        }
    }

    return Some(safe_report_counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
