advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul[(]([0-9]{1,3})[,]([0-9]{1,3})[)]").unwrap();
    Some(re.captures_iter(input).
        map(|c| c.extract()).
        map(|(_, [v1s, v2s])| v1s.parse::<u32>().unwrap() * v2s.parse::<u32>().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {

    let re = Regex::new(r"(?<funcmul>mul)[(]([0-9]{1,3})[,]([0-9]{1,3})[)]|(?<funcdo>do)[(][)]|(?<funcdont>don't)[(][)]").unwrap();
    let mut doval : u32 = 1;
    let mut total : u32 = 0;
    for cap in re.captures_iter(input) {
        println!("{:?}", cap);
        //let func = cap.name("func").unwrap().as_str();
        if cap.name("funcmul").is_some() {
            total += doval * cap.get(2).unwrap().as_str().parse::<u32>().unwrap() * cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
        } else if cap.name("funcdont").is_some() {
            doval = 0;
        } else if cap.name("funcdo").is_some() {
            doval = 1;
        } else {
            unreachable!();
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
