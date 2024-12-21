use pathfinding::prelude::{astar, yen};
use std::collections::HashSet;

advent_of_code::solution!(16);

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

fn get_succ(map: &Vec<Vec<char>>, state: (usize, usize, Dir)) -> Vec<((usize, usize, Dir), u64)> {
    let mut succ: Vec<((usize, usize, Dir), u64)> = Vec::new();
    let (x, y, dir) = state;
    // both 90-degree turns are options
    succ.push(((x, y, dir.turn_left()), 1000));
    succ.push(((x, y, dir.turn_right()), 1000));
    let (off_x, off_y) = dir.next_offset();
    let next_x = x as isize + off_x;
    let next_y = y as isize + off_y;
    let ch = map[next_y as usize][next_x as usize];
    if ch == '.' || ch == 'E' {
        succ.push(((next_x as usize, next_y as usize, dir), 1)); // can go forward one space for 1 pt
    }
    //println!("Successors for {:?} -> {:?}", &state, &succ);
    succ
}

fn get_heur(_map: &Vec<Vec<char>>, end_state: (usize, usize), state: (usize, usize, Dir)) -> u64 {
    // TODO: I think we can add a penalty if we are facing away from the end state to handle direction...remove it if it causes problems.
    let dist = (end_state.0 as u64).abs_diff(state.0 as u64)
        + (end_state.1 as u64).abs_diff(state.1 as u64);
    if dist == 0 {
        return 0;
    }
    let next_step = state.2.next_offset();
    let mut dir_penalty = 0;
    if (next_step.0 < 0 && end_state.0 > state.0) || (next_step.0 > 0 && end_state.0 < state.0) {
        dir_penalty += 1000;
    }
    if (next_step.1 < 1 && end_state.1 > state.1) || (next_step.1 < 1 && end_state.1 > state.1) {
        dir_penalty += 1000;
    }
    //println!("Heuristic for {:?} -> {:?}", &state, dist);
    dist //+ dir_penalty
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let dir = Dir::Right;
    let start_pos = map_find(&map, 'S').unwrap();
    let (start_x, start_y) = start_pos;
    let end_pos = map_find(&map, 'E').unwrap();
    let res = astar(
        &(start_x, start_y, dir),
        |state| get_succ(&map, *state),
        |state| get_heur(&map, end_pos, *state),
        |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
    )
    .unwrap();
    println!("{:?}", res);
    Some(res.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let dir = Dir::Right;
    let start_pos = map_find(&map, 'S').unwrap();
    let (start_x, start_y) = start_pos;
    let end_pos = map_find(&map, 'E').unwrap();
    let res = yen(
        &(start_x, start_y, dir),
        |state| get_succ(&map, *state),
        //|state| get_heur(&map, end_pos, *state),
        |state| (state.0 == end_pos.0 && state.1 == end_pos.1),
        16,
    );
    let best_cst = res[0].1;
    let best_results = res
        .iter()
        .filter(|(pth, cst)| *cst == best_cst)
        .collect::<Vec<_>>();
    println!("Number of minimum-cost paths: {:?}", best_results.len());
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    for r in best_results {
        for (x, y, _) in (*r).0.iter() {
            positions.insert((*x, *y));
        }
    }

    Some(positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
