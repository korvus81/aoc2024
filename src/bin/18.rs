advent_of_code::solution!(18);

use itertools::Itertools;
use pathfinding::prelude::{astar, yen};

fn get_succ(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<((usize, usize), u32)> {
    let mut res: Vec<((usize, usize), u32)> = vec![];
    if x > 0 {
        if map[y][x - 1] == '.' {
            res.push(((x - 1, y), 1));
        };
    }
    if y > 0 {
        if map[y - 1][x] == '.' {
            res.push(((x, y - 1), 1));
        };
    }
    if x < map[y].len() - 1 {
        if map[y][x + 1] == '.' {
            res.push(((x + 1, y), 1));
        };
    }
    if y < map.len() - 1 {
        if map[y + 1][x] == '.' {
            res.push(((x, y + 1), 1));
        };
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let coords = input
        .trim()
        .lines()
        .map(|l| l.split(",").collect_tuple().unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect_vec();
    println!("{:?}", coords);
    let MAX_X = coords.iter().map(|(x, y)| x).max().unwrap().to_owned();
    let MAX_Y = coords.iter().map(|(x, y)| y).max().unwrap().to_owned();
    let START_X = 0 as usize;
    let START_Y = 0 as usize;
    let END_X = MAX_X;
    let END_Y = MAX_Y;
    let mut map = vec![vec!['.'; (MAX_X + 1) as usize]; (MAX_Y + 1) as usize];
    println!("MAX_X: {}", MAX_X);
    let STEPS = if MAX_X == 6 { 12 } else { 1024 };
    for (x, y) in &coords[0..STEPS] {
        map[*y][*x] = '#';
    }
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!("");
    }
    let res = astar(
        &(START_X, START_Y),
        |(x, y)| get_succ(*x, *y, &map),
        |(x, y)| (MAX_X - x) as u32 + (MAX_Y - y) as u32,
        |(x, y)| *x == END_X && *y == END_Y,
    )
    .unwrap();
    println!("res: {:?}", res);
    Some(res.1 as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = input
        .trim()
        .lines()
        .map(|l| l.split(",").collect_tuple().unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect_vec();
    println!("{:?}", coords);
    let MAX_X = coords.iter().map(|(x, y)| x).max().unwrap().to_owned();
    let MAX_Y = coords.iter().map(|(x, y)| y).max().unwrap().to_owned();
    let START_X = 0 as usize;
    let START_Y = 0 as usize;
    let END_X = MAX_X;
    let END_Y = MAX_Y;
    let mut map = vec![vec!['.'; (MAX_X + 1) as usize]; (MAX_Y + 1) as usize];
    println!("MAX_X: {}", MAX_X);

    for (x, y) in coords {
        map[y][x] = '#';
        let res = astar(
            &(START_X, START_Y),
            |(x, y)| get_succ(*x, *y, &map),
            |(x, y)| (MAX_X - x) as u32 + (MAX_Y - y) as u32,
            |(x, y)| *x == END_X && *y == END_Y,
        );
        if res.is_none() {
            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    print!("{}", map[y][x]);
                }
                println!("");
            }
            println!("x,y: {},{}", x, y);
            return Some(format!("{},{}", x, y));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
