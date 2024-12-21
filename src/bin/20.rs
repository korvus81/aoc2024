use pathfinding::prelude::{astar, yen};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashSet;

advent_of_code::solution!(20);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next_offset(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
    fn next_pos(self, x: isize, y: isize, dist: isize) -> (isize, isize) {
        let off = self.next_offset();
        (x + (off.0 * dist), y + (off.1 * dist))
    }
    fn turn_left(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
    fn turn_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.trim()
        .lines()
        .map(|ln| ln.trim().chars().collect())
        .collect()
}
fn map_find(map: &Vec<Vec<char>>, ch: char) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == ch {
                return Some((x, y));
            }
        }
    }
    None
}

fn get_succ(map: &Vec<Vec<char>>, state: (usize, usize, u32)) -> Vec<((usize, usize, u32), u64)> {
    let mut succ: Vec<((usize, usize, u32), u64)> = Vec::new();

    let (x, y, cheats) = state;

    if x > 0 && map[y][x - 1] != '#' {
        succ.push(((x - 1, y, cheats), 1))
    }
    if y > 0 && map[y - 1][x] != '#' {
        succ.push(((x, y - 1, cheats), 1))
    }
    if x < map[0].len() - 1 && map[y][x + 1] != '#' {
        succ.push(((x + 1, y, cheats), 1))
    }
    if y < map.len() - 1 && map[y + 1][x] != '#' {
        succ.push(((x, y + 1, cheats), 1))
    }
    if cheats > 0 {
        if x > 1 && map[y][x - 1] == '#' && map[y][x - 2] != '#' {
            succ.push(((x - 2, y, cheats - 1), 2))
        }
        if x < map[0].len() - 2 && map[y][x + 1] == '#' && map[y][x + 2] != '#' {
            succ.push(((x + 2, y, cheats - 1), 2))
        }
        if y > 1 && map[y - 1][x] == '#' && map[y - 2][x] != '#' {
            succ.push(((x, y - 2, cheats - 1), 2))
        }
        if y < map.len() - 2 && map[y + 1][x] == '#' && map[y + 2][x] != '#' {
            succ.push(((x, y + 2, cheats - 1), 2))
        }
    }
    succ
}

fn get_possible_cheats(map: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let mut cheats: Vec<((usize, usize), (usize, usize))> = vec![];
    // we can ignore starting on edges
    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] != '#' {
                if x > 1 && map[y][x - 1] == '#' && map[y][x - 2] != '#' {
                    cheats.push(((x, y), (x - 2, y)));
                }
                if x < map[0].len() - 2 && map[y][x + 1] == '#' && map[y][x + 2] != '#' {
                    cheats.push(((x, y), (x + 2, y)));
                }
                if y > 1 && map[y - 1][x] == '#' && map[y - 2][x] != '#' {
                    cheats.push(((x, y), (x, y - 2)));
                }
                if y < map.len() - 2 && map[y + 1][x] == '#' && map[y + 2][x] != '#' {
                    cheats.push(((x, y), (x, y + 2)));
                }
            }
        }
    }

    cheats
}

fn get_succ_specific_cheat(
    map: &Vec<Vec<char>>,
    cheat: ((usize, usize), (usize, usize)),
    state: (usize, usize, u32),
) -> Vec<((usize, usize, u32), u64)> {
    let mut succ: Vec<((usize, usize, u32), u64)> = Vec::new();

    let (x, y, cheats) = state;

    if x > 0 && map[y][x - 1] != '#' {
        succ.push(((x - 1, y, cheats), 1))
    }
    if y > 0 && map[y - 1][x] != '#' {
        succ.push(((x, y - 1, cheats), 1))
    }
    if x < map[0].len() - 1 && map[y][x + 1] != '#' {
        succ.push(((x + 1, y, cheats), 1))
    }
    if y < map.len() - 1 && map[y + 1][x] != '#' {
        succ.push(((x, y + 1, cheats), 1))
    }
    if x == cheat.0 .0 && y == cheat.0 .1 {
        succ.push((
            (cheat.1 .0, cheat.1 .1, 0),
            cheat.0 .0.abs_diff(cheat.1 .0) as u64 + cheat.0 .1.abs_diff(cheat.1 .1) as u64,
        ))
    }
    succ
}

fn get_heur(_map: &Vec<Vec<char>>, end_state: (usize, usize), state: (usize, usize, u32)) -> u64 {
    let dist = (end_state.0 as u64).abs_diff(state.0 as u64)
        + (end_state.1 as u64).abs_diff(state.1 as u64);
    dist
}

fn get_paths(
    map: &Vec<Vec<char>>,
    end_state: (usize, usize),
    state: (usize, usize, u32),
    max_cost: u64,
) -> u64 {
    0
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let target_savings: u64;
    if map.len() <= 15 {
        // example
        target_savings = 40u64;
    } else {
        target_savings = 100u64;
    }
    let start_pos = map_find(&map, 'S').unwrap();
    let (start_x, start_y) = start_pos;
    let end_pos = map_find(&map, 'E').unwrap();
    let res = astar(
        &(start_x, start_y, 0),
        |state| get_succ(&map, *state),
        |state| get_heur(&map, end_pos, *state),
        |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
    )
    .unwrap();
    println!("{:?}", res);
    let without_cheats = res.1;
    let target_cost = without_cheats - target_savings;
    //println!("lenth: {}",res.0.len());
    //Some(res.1)
    let cheats = get_possible_cheats(&map);
    println!("cheats count: {:?}", cheats.len());
    let mut good_cheats = 0u64;
    for (cheatnum, cheat) in cheats.iter().enumerate() {
        let res = astar(
            &(start_x, start_y, 1),
            |state| get_succ_specific_cheat(&map, *cheat, *state),
            |state| get_heur(&map, end_pos, *state),
            |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
        );
        if res.unwrap_or((vec![], without_cheats + 1)).1 <= target_cost {
            good_cheats = good_cheats + 1;
        }
        if cheatnum % 100 == 0 {
            println!(
                "[{}/{}] good cheats: {:?}",
                cheatnum,
                cheats.len(),
                good_cheats
            );
        }
    }

    Some(good_cheats)
}

fn get_possible_cheats_20ps(map: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let mut cheats: Vec<((usize, usize), (usize, usize))> = vec![];
    // we can ignore starting on edges
    for y in 1..map.len() as isize - 1 {
        for x in 1..map[0].len() as isize - 1 {
            if map[y as usize][x as usize] != '#' {
                for x2 in (x as isize) - 20..=(x as isize) + 20 {
                    for y2 in (y as isize) - 20..=(y as isize) + 20 {
                        if (x2.abs_diff(x) + y2.abs_diff(y)) <= 20
                            && x2 > 0
                            && y2 > 0
                            && x2 < map[0].len() as isize
                            && y2 < map.len() as isize
                            && map[y2 as usize][x2 as usize] != '#'
                        {
                            // this means it COULD work...now lets see if it is a good idea
                            // if (x2 > x && map[y as usize][x as usize + 1] != '#')
                            //     || (x2 < x && map[y as usize][x as usize - 1] != '#')
                            //     || (y2 > y && map[y as usize + 1 ][x as usize] != '#')
                            //     || (y2 < y && map[y as usize - 1 ][x as usize] != '#')
                            // {
                            //     // lets pass because we can move a space closer
                            // } else {
                            cheats.push(((x as usize, y as usize), (x2 as usize, y2 as usize)));
                            //}
                        }
                    }
                }
            }
        }
    }

    cheats
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let target_savings: u64;
    if map.len() <= 15 {
        // example
        target_savings = 50u64;
    } else {
        target_savings = 100u64;
    }
    let start_pos = map_find(&map, 'S').unwrap();
    let (start_x, start_y) = start_pos;
    let end_pos = map_find(&map, 'E').unwrap();
    let res = astar(
        &(start_x, start_y, 0),
        |state| get_succ(&map, *state),
        |state| get_heur(&map, end_pos, *state),
        |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
    )
    .unwrap();
    println!("{:?}", res);
    let without_cheats = res.1;
    let target_cost = without_cheats - target_savings;
    //println!("lenth: {}",res.0.len());
    //Some(res.1)
    let cheats = get_possible_cheats_20ps(&map);
    println!("cheats count: {:?}", cheats.len());
    let good_cheats = cheats
        .par_iter()
        .map(|cheat| {
            let res = astar(
                &(start_x, start_y, 1),
                |state| get_succ_specific_cheat(&map, *cheat, *state),
                |state| get_heur(&map, end_pos, *state),
                |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
            );
            if res.unwrap_or((vec![], without_cheats + 1)).1 <= target_cost {
                1
            } else {
                0
            }
        })
        .sum::<u64>();

    Some(good_cheats)
}
pub fn part_two_slow(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let target_savings: u64;
    if map.len() <= 15 {
        // example
        target_savings = 50u64;
    } else {
        target_savings = 100u64;
    }
    let start_pos = map_find(&map, 'S').unwrap();
    let (start_x, start_y) = start_pos;
    let end_pos = map_find(&map, 'E').unwrap();
    let res = astar(
        &(start_x, start_y, 0),
        |state| get_succ(&map, *state),
        |state| get_heur(&map, end_pos, *state),
        |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
    )
    .unwrap();
    println!("{:?}", res);
    let without_cheats = res.1;
    let target_cost = without_cheats - target_savings;
    //println!("lenth: {}",res.0.len());
    //Some(res.1)
    let cheats = get_possible_cheats_20ps(&map);
    println!("cheats count: {:?}", cheats.len());
    let mut good_cheats = 0u64;
    for (cheatnum, cheat) in cheats.iter().enumerate() {
        let res = astar(
            &(start_x, start_y, 1),
            |state| get_succ_specific_cheat(&map, *cheat, *state),
            |state| get_heur(&map, end_pos, *state),
            |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
        );
        if res.unwrap_or((vec![], without_cheats + 1)).1 <= target_cost {
            good_cheats = good_cheats + 1;
        }
        if cheatnum % 100 == 0 {
            println!(
                "[{}/{}] good cheats: {:?}",
                cheatnum,
                cheats.len(),
                good_cheats
            );
        }
    }

    Some(good_cheats)
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
        assert_eq!(result, Some(285));
    }
}
