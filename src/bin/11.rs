use itertools::Itertools;
use memoize::memoize;
advent_of_code::solution!(11);

fn even_digits(numin: u64) -> bool {
    let st = numin.to_string();
    st.len() % 2 == 0
}

fn split(numin:u64) -> (u64, u64) {
    let st = numin.to_string();
    (st[0..&st.len()/2].parse::<u64>().unwrap(), st[&st.len()/2..].parse::<u64>().unwrap())
}


#[memoize]
fn next(numin: u64) -> Vec<u64> {
    if numin == 0 {
        return vec![1];
    } else if even_digits(numin) {
        let splitted = split(numin);
        return vec![splitted.0, splitted.1];
    } else {
        return vec![numin*2024];
    }
}

#[memoize]
fn stones_at_end(numin: u64, steps: i32) -> u64 {
    if steps == 0 {
        return 1
    } else {

    }
        if numin == 0 {
            return stones_at_end(1, steps-1);
        } else if even_digits(numin) {
            let splitted = split(numin);
            return stones_at_end(splitted.0,steps-1) + stones_at_end(splitted.1,steps-1);
        } else {
            return stones_at_end(numin*2024, steps-1);
        }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones : Vec<u64>  = input.trim().split_whitespace().map(|nst| nst.parse::<u64>().unwrap()).collect();
    //println!("Initial: {:?}", stones);
    for i in 0..25 {
        stones = stones.iter().flat_map(|s| next(*s)).collect_vec();
        //println!("[{}] {:?}", i, stones);
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones : Vec<u64>  = input.trim().split_whitespace().map(|nst| nst.parse::<u64>().unwrap()).collect();
    let mut count = 0;
    for s in stones.iter() {
        let atend = stones_at_end(*s,75);
        println!("{:?} => {:?} stones",s, atend);
        count += atend;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
