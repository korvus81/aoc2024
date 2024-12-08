use std::iter;
use itertools::Itertools;
//use std::iter::repeat_n;
advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let equations = input
        .trim()
        .lines()
        .map(|ln| ln.split_once(":").unwrap())
        .map(|(tot, nums)| {
            (
                tot.parse::<u64>().unwrap(),
                nums.trim().split(" ")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();
    equations
}


pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let mut sum:u64 = 0;
    for (tot,nums) in equations.iter() {
        if (can_work(tot,nums)) {
            sum += *tot as u64;
        }
    }

    Some(sum)
}


fn can_work(tot: &u64, nums: &Vec<u64>) -> bool {
    let possible_ops = iter::repeat_n(["+","*"],nums.len()-1).multi_cartesian_product();//.percombinations_with_replacement(nums.len()-1);
    for ops in possible_ops {
        let mut acc = nums[0];
        for (i,num) in nums[1..].iter().enumerate() {
            match ops[i] {
                "+" => acc += num,
                "*" => acc *= num,
                _ => unreachable!(),
            }
            if acc > *tot {
                break; // it will only get larger
            }
        }
        // if *tot == 292 {
        //     println!("checking {} == {:?}",tot, nums.iter().map(|n| n.to_string()).interleave(ops.iter().map(|ch|ch.to_string())).collect_vec());
        // }
        if acc == *tot {
            println!("{} == {:?}",tot, nums.iter().map(|n| n.to_string()).interleave(ops.iter().map(|ch|ch.to_string())).collect_vec());
            return true;
        }
    }
    false
}


fn can_work2(tot: &u64, nums: &Vec<u64>) -> bool {
    let possible_ops = iter::repeat_n(["+","*","||"],nums.len()-1).multi_cartesian_product();//.percombinations_with_replacement(nums.len()-1);
    for ops in possible_ops {
        let mut acc = nums[0];
        for (i,num) in nums[1..].iter().enumerate() {
            match ops[i] {
                "+" => acc += num,
                "*" => acc *= num,
                "||" => acc = (acc.to_string()+&num.to_string()).parse::<u64>().unwrap(),
                _ => unreachable!(),
            }
            if acc > *tot {
                break; // it will only get larger
            }
        }
        // if *tot == 292 {
        //     println!("checking {} == {:?}",tot, nums.iter().map(|n| n.to_string()).interleave(ops.iter().map(|ch|ch.to_string())).collect_vec());
        // }
        if acc == *tot {
            println!("{} == {:?}",tot, nums.iter().map(|n| n.to_string()).interleave(ops.iter().map(|ch|ch.to_string())).collect_vec());
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let mut sum:u64 = 0;
    for (tot,nums) in equations.iter() {
        if (can_work2(tot,nums)) {
            sum += *tot as u64;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
