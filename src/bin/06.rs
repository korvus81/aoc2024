use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

advent_of_code::solution!(6);

fn parse_board(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|ln| ln.trim().chars().collect())
        .collect()
}

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
        // match self {
        //     Dir::Up => (x, y+ (-1 * dist)),
        //     Dir::Down => (x, 1 * dist),
        //     Dir::Left => (-1 * dist, y),
        //     Dir::Right => (1 * dist, y),
        // }
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

fn is_guard(ch: &char) -> bool {
    *ch == '^' || *ch == '<' || *ch == '>' || *ch == 'v'
}

fn get_guard_dir(ch: &char) -> Option<Dir> {
    match ch {
        '^' => Some(Dir::Up),
        '<' => Some(Dir::Left),
        '>' => Some(Dir::Right),
        'v' => Some(Dir::Down),
        _ => None,
    }
}

fn get_guard(board: &Vec<Vec<char>>) -> (isize, isize, Dir) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if is_guard(&board[y][x]) {
                return (x as isize, y as isize, get_guard_dir(&board[y][x]).unwrap());
            }
        }
    }
    unreachable!();
}

fn draw_board(board: &Vec<Vec<char>>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            let ch = board[y][x];
            match ch {
                '.' => stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                    .unwrap(),
                '#' => stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                    .unwrap(),
                'O' => stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                    .unwrap(),
                '^' | '<' | '>' | 'v' => stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap(),
                _ => stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                    .unwrap(),
            }
            write!(&mut stdout, "{}", ch).unwrap();
        }
        stdout.reset();
        writeln!(&mut stdout, "").unwrap();
    }
    stdout.reset();
    writeln!(&mut stdout, "").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_board(input);
    let width = board[0].len() as isize;
    let height = board.len() as isize;
    let (mut guard_x, mut guard_y, mut guard_dir) = get_guard(&board);
    draw_board(&board);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((guard_x, guard_y));

    // initialize this to current position so we can enter the loop
    let mut next_x = guard_x;
    let mut next_y = guard_y;
    while next_x >= 0 && next_y >= 0 && next_x < width && next_y < height {
        let next_ch = board[next_y as usize][next_x as usize];
        println!(
            "({},{}) @ {:?} -> ({},{}) '{}'",
            guard_x, guard_y, guard_dir, next_x, next_y, next_ch
        );
        if next_ch == '#' {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
        visited.insert((guard_x, guard_y));
        (next_x, next_y) = guard_dir.next_pos(guard_x, guard_y, 1);
        println!(
            "...({},{}) @ {:?} -> ({},{}) '{}'",
            guard_x, guard_y, guard_dir, next_x, next_y, next_ch
        );
    }
    Some(visited.len() as u32)
}

fn get_visited_set(
    board: &Vec<Vec<char>>,
    mut guard_x: isize,
    mut guard_y: isize,
    mut guard_dir: Dir,
) -> (HashSet<(isize, isize, Dir)>, bool) {
    let height: isize = board.len() as isize;
    let width: isize = board[0].len() as isize;

    let mut visited: HashSet<(isize, isize, Dir)> = HashSet::new();
    //visited.insert((guard_x, guard_y, guard_dir)); // not doing this because I'll think I'm in a loop on the first iteration

    // initialize this to current position so we can enter the loop
    let mut next_x = guard_x;
    let mut next_y = guard_y;
    while next_x >= 0 && next_y >= 0 && next_x < width && next_y < height {
        let next_ch = board[next_y as usize][next_x as usize];
        if next_ch == '#' || next_ch == 'O' {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
        let entry = (guard_x, guard_y, guard_dir);
        if visited.contains(&entry) {
            // we found a loop!
            return (visited, true); // true = found loop
        }
        visited.insert(entry);
        (next_x, next_y) = guard_dir.next_pos(guard_x, guard_y, 1);
    }
    (visited, false) // false = no loop
}
fn get_visited_list(
    board: &Vec<Vec<char>>,
    mut guard_x: isize,
    mut guard_y: isize,
    mut guard_dir: Dir,
) -> (Vec<(isize, isize)>, bool) {
    let (visited_set, is_loop) = get_visited_set(&board, guard_x, guard_y, guard_dir);
    let visited_locs_set = visited_set
        .iter()
        .map(|(x, y, dir)| (*x, *y))
        .collect::<HashSet<(isize, isize)>>();
    (
        visited_locs_set.iter().map(|(x, y)| (*x, *y)).collect_vec(),
        is_loop,
    )
}

fn board_with_obstacle(board: &Vec<Vec<char>>, obs_x: usize, obs_y: usize) -> Vec<Vec<char>> {
    let mut new_board: Vec<Vec<char>> = Vec::new();
    for y in 0..board.len() {
        if obs_y == y {
            let mut row: Vec<char> = Vec::new();
            for x in 0..board[y].len() {
                if obs_x != x {
                    row.push(board[y][x]);
                } else {
                    row.push('O');
                }
            }
            new_board.push(row);
        } else {
            let row = board[y].clone();
            new_board.push(row);
        }
    }
    new_board
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_board(input);
    let width = board[0].len() as isize;
    let height = board.len() as isize;
    let (guard_x, guard_y, guard_dir) = get_guard(&board);
    draw_board(&board);

    let (visited,is_loop) = get_visited_list(&board, guard_x.clone(), guard_y.clone(), guard_dir.clone());
    assert!(!is_loop);

    let mut possible_location_count:u32 = 0;
    for (i,(vis_x,vis_y)) in visited.iter().enumerate() {
        //println!("[{}] @ ({},{})", i, vis_x,vis_y);
        let new_board = board_with_obstacle(&board,*vis_x as usize,*vis_y as usize);
        let (v2, is_loop2) = get_visited_list(&new_board, guard_x.clone(), guard_y.clone(), guard_dir.clone());
        if is_loop2 {
            possible_location_count += 1;
        }
    }


    Some(possible_location_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
