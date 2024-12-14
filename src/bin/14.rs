use regex::Regex;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(14);

fn parse_line(ln: &str) -> ((i64, i64), (i64, i64)) {
    let LINE_RE: Regex =
        Regex::new("p=([-]?[0-9]+),([-]?[0-9]+) v=([-]?[0-9]+),([-]?[0-9]+)").unwrap();
    let caps = LINE_RE.captures(&ln).unwrap();
    let pos = (
        caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    );
    let vel = (
        caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
    );
    (pos, vel)
}

fn sim_robots(
    robots: &Vec<((i64, i64), (i64, i64))>,
    map_width: i64,
    map_height: i64,
    steps: i64,
) -> Vec<((i64, i64), (i64, i64))> {
    robots
        .iter()
        .map(|((x, y), (vx, vy))| {
            (
                (
                    (*x + (*vx * steps)) % map_width,
                    (*y + (*vy * steps)) % map_height,
                ),
                (*vx, *vy),
            )
        })
        .map(|((x, y), (vx, vy))| {
            // normalize so no negative coordinates
            (
                (
                    if x >= 0 { x } else { map_width + x },
                    if y >= 0 { y } else { map_height + y },
                ),
                (vx, vy),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = input
        .trim()
        .lines()
        .map(|ln| parse_line(&ln))
        .collect::<Vec<_>>();

    let (MAP_WIDTH, MAP_HEIGHT): (i64, i64) = if robots.len() == 12 {
        // example input
        (11, 7)
    } else {
        // real input
        (101, 103)
    };
    println!("[({}x{})] {:?}", MAP_WIDTH, MAP_HEIGHT, robots);

    let new_robots = sim_robots(&robots, MAP_WIDTH, MAP_HEIGHT, 100);
    println!("[({}x{})] {:?}", MAP_WIDTH, MAP_HEIGHT, new_robots);
    let mut quad1cnt = 0;
    let mut quad2cnt = 0;
    let mut quad3cnt = 0;
    let mut quad4cnt = 0;
    for ((x, y), (_, _)) in new_robots {
        if x < MAP_WIDTH / 2 {
            if y < MAP_HEIGHT / 2 {
                quad1cnt += 1;
            } else if y > MAP_HEIGHT / 2 {
                quad3cnt += 1;
            }
        } else if x > MAP_WIDTH / 2 {
            if y < MAP_HEIGHT / 2 {
                quad2cnt += 1;
            } else if y > MAP_HEIGHT / 2 {
                quad4cnt += 1;
            } // ignore == MAP_HEIGHT
        }
    }

    Some(quad1cnt * quad2cnt * quad3cnt * quad4cnt)
}

// 500 robots, I assume a Christmas tree is horizontally symmetrical and is roughly triangle-shaped.  Probably in unique locations.
fn check_for_xmas(robots: &Vec<((i64, i64), (i64, i64))>, map_width: i64, map_height: i64) -> bool {
    check_for_xmas_unique(robots, map_width, map_height)
        || check_for_xmas_clumped(robots, map_width, map_height)
}

fn check_for_xmas_unique(
    robots: &Vec<((i64, i64), (i64, i64))>,
    map_width: i64,
    map_height: i64,
) -> bool {
    // lets check for unique locations
    let mut locs: HashSet<(i64, i64)> = HashSet::new();
    for ((x, y), (_, _)) in robots {
        if locs.contains(&(*x, *y)) {
            return false;
        } else {
            locs.insert((*x, *y));
        }
    }
    return true;
}
fn check_for_xmas_clumped(
    robots: &Vec<((i64, i64), (i64, i64))>,
    map_width: i64,
    map_height: i64,
) -> bool {
    let mut count = 0;
    for i in 0..robots.len() - 1 {
        for j in i + 1..robots.len() {
            if robots[i].0 .0.abs_diff(robots[j].0 .0) <= 1
                && robots[i].0 .1.abs_diff(robots[j].0 .1) <= 1
            {
                count += 1;
            }
        }
    }
    count > 350 // arbitrary level -- if we have 500 robots, assume a majority are touching if we are making an image?
}

fn visualize(robots: &Vec<((i64, i64), (i64, i64))>, map_width: i64, map_height: i64) {
    for y in 0..map_height {
        for x in 0..map_width {
            let num = robots
                .iter()
                .filter(|((rx, ry), (_, _))| *rx == x && *ry == y)
                .count();
            if num == 0 {
                print!(".")
            } else if num < 10 {
                print!("{}", num);
            } else {
                print!("*");
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let robots = input
        .trim()
        .lines()
        .map(|ln| parse_line(&ln))
        .collect::<Vec<_>>();

    let (MAP_WIDTH, MAP_HEIGHT): (i64, i64) = if robots.len() == 12 {
        // example input
        (11, 7)
    } else {
        // real input
        (101, 103)
    };
    println!("[({}x{})] {:?}", MAP_WIDTH, MAP_HEIGHT, robots);
    let mut stepcount = 1u64;
    let mut new_robots = sim_robots(&robots, MAP_WIDTH, MAP_HEIGHT, 1);
    while !check_for_xmas(&new_robots, MAP_WIDTH, MAP_HEIGHT) {
        new_robots = sim_robots(&new_robots, MAP_WIDTH, MAP_HEIGHT, 1);
        stepcount += 1;
        if stepcount % 1000 == 0 {
            println!("{} steps", stepcount);
            visualize(&new_robots, MAP_WIDTH, MAP_HEIGHT);
        }
    }
    visualize(&new_robots, MAP_WIDTH, MAP_HEIGHT);

    Some(stepcount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
