use std::cmp::min;

advent_of_code::solution!(15);

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

fn parse_input(s: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];
    let mut in_moves: bool = false;
    for ln in s.trim().lines() {
        if !in_moves {
            if ln.trim().len() == 0 {
                in_moves = true;
            } else {
                map.push(ln.trim().chars().collect());
            }
        } else {
            if ln.trim().len() > 0 {
                moves.extend(ln.trim().chars());
            }
        }
    }

    (map, moves)
}

fn make_move(
    map: &Vec<Vec<char>>,
    robot_x: i32,
    robot_y: i32,
    mv: char,
) -> (Vec<Vec<char>>, i32, i32) {
    let mut new_map = map.clone();
    let d = match mv {
        '^' => Dir::Up,
        'v' => Dir::Down,
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => unreachable!(),
    };
    let (mut robot_x2, mut robot_y2) = d.next_pos(robot_x as isize, robot_y as isize, 1);
    match new_map[robot_y2 as usize][robot_x2 as usize] {
        '.' => {
            new_map[robot_y2 as usize][robot_x2 as usize] = '@';
            new_map[robot_y as usize][robot_x as usize] = '.';
            (new_map, robot_x2 as i32, robot_y2 as i32)
        }
        '#' => {
            // if it is a wall. nothing happens
            (new_map, robot_x, robot_y)
        }
        'O' => {
            let (mut x, mut y) = (robot_x2, robot_y2);
            while new_map[y as usize][x as usize] == 'O' {
                (x, y) = d.next_pos(x as isize, y as isize, 1);
            }
            if new_map[y as usize][x as usize] == '#' {
                // do nothing
                (new_map, robot_x, robot_y)
            } else if new_map[y as usize][x as usize] == '.' {
                new_map[y as usize][x as usize] = 'O'; // replace space with box
                new_map[robot_y as usize][robot_x as usize] = '.'; // replace current robot with empty space
                new_map[robot_y2 as usize][robot_x2 as usize] = '@'; // put robot in new position -- box that was there moved one space and eventually filled the empty spot we found
                (new_map, robot_x2 as i32, robot_y2 as i32)
            } else {
                println!(
                    "Unknown case ({}) from running into O {} @ (x:{}, y:{}) (robot at x:{}, y:{})",
                    new_map[y as usize][x as usize], mv, robot_x2, robot_y2, robot_x, robot_y
                );
                unreachable!()
            }
        }
        _ => {
            println!(
                "Unknown map spot {} @ (x:{}, y:{}) (robot at x:{}, y:{})",
                mv, robot_x2, robot_y2, robot_x, robot_y
            );
            unreachable!()
        }
    }
}

fn calc_gps(map: &Vec<Vec<char>>) -> u64 {
    let mut gps = 0u64;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                gps += (100 * y as u64) + x as u64;
            }
        }
    }
    gps
}

fn display_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (map, moves) = parse_input(input);
    println!("Map: {:?}", map);
    println!("Moves: {:?}", moves);
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                robot_x = x as i32;
                robot_y = y as i32;
            }
        }
    }

    let mut cur_map = map.clone();
    for mv in moves.iter() {
        display_map(&cur_map);
        (cur_map, robot_x, robot_y) = make_move(&cur_map, robot_x, robot_y, *mv);
    }
    let gps = calc_gps(&cur_map);
    Some(gps)
}

fn scale_up_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = vec![];
    for y in 0..map.len() {
        let mut new_row: Vec<char> = vec![];
        for x in 0..map[y].len() {
            match map[y][x] {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => unreachable!(),
            }
        }
        new_map.push(new_row);
    }
    new_map
}

fn make_move2(
    map: &Vec<Vec<char>>,
    robot_x: i32,
    robot_y: i32,
    mv: char,
) -> (Vec<Vec<char>>, i32, i32) {
    let mut new_map = map.clone();
    let d = match mv {
        '^' => Dir::Up,
        'v' => Dir::Down,
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => unreachable!(),
    };
    let (mut robot_x2, mut robot_y2) = d.next_pos(robot_x as isize, robot_y as isize, 1);
    match new_map[robot_y2 as usize][robot_x2 as usize] {
        '.' => {
            new_map[robot_y2 as usize][robot_x2 as usize] = '@';
            new_map[robot_y as usize][robot_x as usize] = '.';
            (new_map, robot_x2 as i32, robot_y2 as i32)
        }
        '#' => {
            // if it is a wall. nothing happens
            (new_map, robot_x, robot_y)
        }
        '[' | ']' => {
            if d == Dir::Left || d == Dir::Right {
                let (mut x, mut y) = (robot_x2, robot_y2);
                while new_map[y as usize][x as usize] == '['
                    || new_map[y as usize][x as usize] == ']'
                {
                    (x, y) = d.next_pos(x as isize, y as isize, 1);
                }
                if new_map[y as usize][x as usize] == '#' {
                    // do nothing
                    (new_map, robot_x, robot_y)
                } else if new_map[y as usize][x as usize] == '.' {
                    if d == Dir::Right {
                        println!("Dir:{:?} robot_y2:{} row={:?}, src={:?} dst={:?}",d,robot_y2, new_map[robot_y2 as usize], robot_x2 as usize..x as usize, robot_x2+1);
                        new_map[robot_y2 as usize].copy_within(robot_x2 as usize..x as usize, (robot_x2+1) as usize);
                    } else { // d == Dir::Left
                        println!("Dir:{:?} robot_y2:{} row={:?}, src={:?} dst={:?}",d,robot_y2, new_map[robot_y2 as usize], (x+1) as usize..=robot_x2 as usize, x);
                        new_map[robot_y2 as usize].copy_within((x+1) as usize..=robot_x2 as usize, x as usize);
                    }
                    new_map[robot_y2 as usize][robot_x2 as usize] = '@'; // new position
                    new_map[robot_y as usize][robot_x as usize] = '.'; // old position

                    (new_map, robot_x2 as i32, robot_y2 as i32)
                } else {
                    println!("Unknown case ({}) from running into O {} @ (x:{}, y:{}) (robot at x:{}, y:{})", new_map[y as usize][x as usize], mv, robot_x2, robot_y2, robot_x, robot_y);
                    unreachable!()
                }
            } else { // Up or Down
                // TODO: this is the hard part.  Maybe a recursive function that will tell me all the moves that will happen for the left and right side of the box?
                let (mut box_x, mut box_y) = (robot_x2, robot_y2);
                if new_map[robot_y2 as usize][robot_x2 as usize] == ']' {
                    // box_x should be the left side of the box
                    // if we are interacting with the right side, reduce box_x by 1
                    box_x = box_x-1;
                }
                if can_move_box(&new_map, box_x, box_y, d) {
                    println!("Can move box at x={},y={}, d={:?}",box_x, box_y, d);
                    move_a_box(&mut new_map, box_x, box_y, d);
                    new_map[robot_y2 as usize][robot_x2 as usize] = '@'; // new position
                    new_map[robot_y as usize][robot_x as usize] = '.'; // old position
                    return (new_map, robot_x2 as i32, robot_y2 as i32);
                } else {
                    return (new_map, robot_x as i32, robot_y as i32);
                }
            }
        }
        _ => {
            println!(
                "Unknown map spot {} @ (x:{}, y:{}) (robot at x:{}, y:{})",
                mv, robot_x2, robot_y2, robot_x, robot_y
            );
            unreachable!()
        }
    }
}

fn can_move_box(map: &Vec<Vec<char>>, box_x: isize, box_y: isize, d: Dir) -> bool {
    // double check we didn't screw something up
    assert_eq!(map[box_y as usize][box_x as usize], '[');
    assert_eq!(map[box_y as usize][box_x as usize+1], ']');
    let (next_x_left,next_y) = d.next_pos(box_x,box_y,1);
    let next_x_right = next_x_left + 1;
    if map[next_y as usize][next_x_left as usize] == '.' && map[next_y as usize][next_x_right as usize] == '.' {
        return true; // all open space
    }
    if map[next_y as usize][next_x_left as usize] == '#' || map[next_y as usize][next_x_right as usize] == '#' {
        return false; // at least one wall in the way
    }
    if map[next_y as usize][next_x_left as usize] == '[' && map[next_y as usize][next_x_right as usize] == ']' { // shouldn't need second condition
        println!("Box (found left side below left) at x={},y={}, checking if it can move {:?}",next_x_left,next_y,d);
        return can_move_box(&map, next_x_left, next_y, d);
    }
    if map[next_y as usize][next_x_left as usize] == ']'  {
        println!("Box (found right side below left) at x={},y={}, checking if it can move {:?}",next_x_left-1,next_y,d);
        let left_ok =  can_move_box(&map, next_x_left-1, next_y, d); // all open space
        let right_ok = map[next_y as usize][next_x_right as usize] == '.' || (map[next_y as usize][next_x_right as usize] == '[' &&  can_move_box(&map, next_x_right, next_y, d));
        return left_ok && right_ok;
    }
    if map[next_y as usize][next_x_left as usize] == '.' && map[next_y as usize][next_x_right as usize] == '[' {
        println!("Box (found left side below right) at x={},y={}, checking if it can move {:?}",next_x_right, next_y,d);
        return can_move_box(&map, next_x_right, next_y, d);
    }
    println!("ch found: {:?} next_y={}, next_x_left={}", map[next_y as usize][next_x_left as usize], next_y, next_x_left);
    unreachable!(); // I think this is true
}

fn move_a_box(map: &mut Vec<Vec<char>>, box_x: isize, box_y: isize, d: Dir) -> bool {
    // double check we didn't screw something up
    assert_eq!(map[box_y as usize][box_x as usize], '[');
    assert_eq!(map[box_y as usize][box_x as usize+1], ']');
    let (next_x_left,next_y) = d.next_pos(box_x,box_y,1);
    let next_x_right = next_x_left + 1;
    println!("Moving box @ x={},y={} -> next_x_left = {}..right={}, next_y={}, d={:?}",box_x,box_y,next_x_left,next_x_right,next_y, d);

    if map[next_y as usize][next_x_left as usize] == '#' || map[next_y as usize][next_x_right as usize] == '#' {
        unreachable!();
        return false; // at least one wall in the way
    }
    if map[next_y as usize][next_x_left as usize] == '[' && map[next_y as usize][next_x_right as usize] == ']' { // shouldn't need second condition
        if can_move_box(&mut *map, next_x_left, next_y, d) { // all open space
            move_a_box(&mut *map, next_x_left, next_y, d);
            map[next_y as usize][next_x_left as usize] = '[';
            map[next_y as usize][next_x_right as usize] = ']';
            map[box_y as usize][box_x as usize] = '.';
            map[box_y as usize][box_x as usize+1] = '.';
            return true;
        }
    }
    if map[next_y as usize][next_x_left as usize] == ']'  {
        let left_ok =  can_move_box(&map, next_x_left-1, next_y, d); // all open space
        if left_ok && map[next_y as usize][next_x_right as usize] == '.' {
            move_a_box(&mut *map, next_x_left-1, next_y, d);
            map[next_y as usize][next_x_left as usize] = '[';
            map[next_y as usize][next_x_right as usize] = ']';
            map[box_y as usize][box_x as usize] = '.';
            map[box_y as usize][box_x as usize+1] = '.';
            return true;
        } else if left_ok && (map[next_y as usize][next_x_right as usize] == '[' &&  can_move_box(&map, next_x_right, next_y, d)) {
            move_a_box(&mut *map, next_x_left-1, next_y, d);
            move_a_box(&mut *map, next_x_right, next_y, d);
            map[next_y as usize][next_x_left as usize] = '[';
            map[next_y as usize][next_x_right as usize] = ']';
            map[box_y as usize][box_x as usize] = '.';
            map[box_y as usize][box_x as usize+1] = '.';
            return true;
        } else {
            unreachable!();
            return false;
        }
    }
    if map[next_y as usize][next_x_left as usize] == '.' && map[next_y as usize][next_x_right as usize] == '[' {
        if can_move_box(&map, next_x_right, next_y, d) {
            move_a_box(&mut *map, next_x_right, next_y, d);
            map[next_y as usize][next_x_left as usize] = '[';
            map[next_y as usize][next_x_right as usize] = ']';
            map[box_y as usize][box_x as usize] = '.';
            map[box_y as usize][box_x as usize+1] = '.';
            return true;
        } else {
            unreachable!();
            return false;
        }
    }
    if map[next_y as usize][next_x_left as usize] == '.' && map[next_y as usize][next_x_right as usize] == '.' {
        map[next_y as usize][next_x_left as usize] = '[';
        map[next_y as usize][next_x_right as usize] = ']';
        map[box_y as usize][box_x as usize] = '.';
        map[box_y as usize][box_x as usize+1] = '.';
        return true; // all open space
    }

    unreachable!(); // I think this is true
}

fn map_is_valid(map: &Vec<Vec<char>>) -> bool {
    let mut saw_left_bracket_last = false;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '[' {
                if saw_left_bracket_last {
                    return false; // two in a row
                }
                saw_left_bracket_last = true;
            } else if map[y][x] == ']' {
                if !saw_left_bracket_last {
                    return false;
                }
                saw_left_bracket_last = false;
            } else if saw_left_bracket_last {
                return false;
            }


        }
    }
    true
}

fn calc_gps2(map: &Vec<Vec<char>>) -> u64 {
    let mut gps = 0u64;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '[' {
                gps += (100 * y as u64) + x as u64;
            }
        }
    }
    gps
}

pub fn part_two(input: &str) -> Option<u64> {
    let (tmpmap, moves) = parse_input(input);
    let map = scale_up_map(&tmpmap);
    println!("Map: {:?}", map);
    println!("Moves: {:?}", moves);
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                robot_x = x as i32;
                robot_y = y as i32;
            }
        }
    }

    let mut cur_map = map.clone();
    for mv in moves.iter() {
        println!("");
        println!("Moving: {:?}", mv);
        display_map(&cur_map);
        assert!(map_is_valid(&cur_map));
        (cur_map, robot_x, robot_y) = make_move2(&cur_map, robot_x, robot_y, *mv);
    }
    let gps = calc_gps2(&cur_map);
    Some(gps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
