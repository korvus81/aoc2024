use std::collections::HashSet;

advent_of_code::solution!(10);
fn parse_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|ln| ln.trim().chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_paths(map: &Vec<Vec<u32>>, cur_path: Vec<(isize,isize)>) -> Vec<(isize,isize)> {
    let (cur_row,cur_col) = cur_path.last().unwrap();
    let cur_val = map[*cur_row as usize][*cur_col as usize];
    let mut reachable: Vec<(isize,isize)> = Vec::new();
    for (next_row,next_col) in [(*cur_row -1,*cur_col ), (*cur_row+1,*cur_col), (*cur_row,*cur_col-1), (*cur_row,*cur_col+1)].iter() {
        if *next_row >=0 && *next_row < map.len() as isize && *next_col>=0 && *next_col < map[0].len() as isize && !cur_path.contains(&(*next_row,*next_col)) {
            let next_val = map[*next_row as usize][*next_col as usize];

            if next_val == cur_val + 1 {
                if next_val == 9 {
                    reachable.push((*next_row,*next_col));
                } else {
                    let mut new_path = cur_path.clone();
                    new_path.push((*next_row, *next_col));
                    let new_reachable = find_paths(&map, new_path);
                    reachable.extend(new_reachable);
                }
            }
        }
    }
    reachable
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let mut trailheads:Vec<(usize,usize)> = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 0 {
                trailheads.push((row,col));
            }
        }
    }
    println!("{:?}", trailheads);
    let mut score = 0;
    for (row,col) in trailheads {
        let reachable = find_paths(&map, vec![(row as isize,col as isize)]);
        let reachable_set: HashSet<(isize,isize)> = HashSet::from_iter(reachable.iter().map(|x| *x));
        score += reachable_set.len();
        println!("trailhead: {:?}, reachable: {:?} ({}), running score: {}", (row,col), reachable_set, reachable_set.len(), score);
    }
    Some(score as u32)
}

fn find_paths2(map: &Vec<Vec<u32>>, cur_path: Vec<(isize,isize)>) -> u64 {
    let (cur_row,cur_col) = cur_path.last().unwrap();
    let cur_val = map[*cur_row as usize][*cur_col as usize];
    let mut path_count = 0;
    for (next_row,next_col) in [(*cur_row -1,*cur_col ), (*cur_row+1,*cur_col), (*cur_row,*cur_col-1), (*cur_row,*cur_col+1)].iter() {
        if *next_row >=0 && *next_row < map.len() as isize && *next_col>=0 && *next_col < map[0].len() as isize && !cur_path.contains(&(*next_row,*next_col)) {
            let next_val = map[*next_row as usize][*next_col as usize];

            if next_val == cur_val + 1 {
                if next_val == 9 {
                    path_count += 1;
                } else {
                    let mut new_path = cur_path.clone();
                    new_path.push((*next_row, *next_col));
                    path_count += crate::find_paths2(&map, new_path);
                }
            }
        }
    }

    path_count
}
pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let mut trailheads:Vec<(usize,usize)> = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 0 {
                trailheads.push((row,col));
            }
        }
    }
    println!("{:?}", trailheads);
    let mut score = 0u64;
    for (row,col) in trailheads {
        let path_count = find_paths2(&map, vec![(row as isize,col as isize)]);
        score += path_count;
        println!("trailhead: {:?}, path_count: {}, running score: {}", (row,col), path_count, score);
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
