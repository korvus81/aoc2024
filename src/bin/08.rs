use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

advent_of_code::solution!(8);

fn parse_board(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|ln| ln.trim().chars().collect())
        .collect()
}

fn get_coords_by_freq(radio_map: &Vec<Vec<char>>) -> HashMap<char, HashSet<(isize, isize)>> {
    let mut coords_by_freq: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();
    for (y, row) in radio_map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch != '.' {
                coords_by_freq
                    .entry(*ch)
                    .or_insert(HashSet::new())
                    //.or_default()
                    .insert((x as isize, y as isize));
            }
        }
    }
    coords_by_freq
}

pub fn part_one(input: &str) -> Option<u32> {
    let radio_map = parse_board(input);
    let rows = radio_map.len() as isize;
    let cols = radio_map[0].len() as isize;
    let coords_by_freq = get_coords_by_freq(&radio_map);
    println!("{:?}", coords_by_freq);
    let mut antinode_coords: HashSet<(isize, isize)> = HashSet::new();
    for ch in coords_by_freq.keys() {
        let coords = &coords_by_freq[ch];
        coords
            .iter()
            .combinations(2)
            .map(|vec| (*vec[0], *vec[1]))
            .for_each(|((x1, y1), (x2, y2))| {
                let x_dist = (x2 as isize) - (x1 as isize);
                let y_dist = (y2 as isize) - (y1 as isize);
                let poss_fwd = (x2 + x_dist, y2 + y_dist);
                let poss_bwd = (x1 - x_dist, y1 - y_dist);
                println!(
                    "[{}] {:?} delta {:?} -> {:?} or {:?}",
                    ch,
                    [(x1, y1), (x2, y2)],
                    (x_dist, y_dist),
                    poss_fwd,
                    poss_bwd
                );
                if !(poss_fwd.0 < 0 || poss_fwd.1 < 0 || poss_fwd.0 >= cols || poss_fwd.1 >= rows) {
                    antinode_coords.insert(poss_fwd);
                };
                if !(poss_bwd.0 < 0 || poss_bwd.1 < 0 || poss_bwd.0 >= cols || poss_bwd.1 >= rows) {
                    antinode_coords.insert(poss_bwd);
                };
            })
    }
    println!("{:?}", antinode_coords);
    Some(antinode_coords.len() as u32)
}

fn draw_map(mapdata: &Vec<Vec<char>>, coords: &HashSet<(isize, isize)>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for y in 0..mapdata.len() {
        for x in 0..mapdata[y].len() {
            let ch = mapdata[y][x];
            match ch {
                '.' => if coords.contains(&(x as isize, y as isize)) {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                        .unwrap();
                    write!(&mut stdout, "#").unwrap();
                } else {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                        .unwrap();
                    write!(&mut stdout, "{}", ch).unwrap();
                },

                _ => {
                    if coords.contains(&(x as isize, y as isize)) {
                        stdout
                            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                            .unwrap();
                        write!(&mut stdout, "{}", ch).unwrap();
                    } else {
                        stdout
                            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                            .unwrap();
                        write!(&mut stdout, "{}", ch).unwrap();
                    }

                }
            }

        }
        stdout.reset();
        writeln!(&mut stdout, "").unwrap();
    }
    stdout.reset();
    writeln!(&mut stdout, "").unwrap();
}
pub fn part_two(input: &str) -> Option<u32> {
    let radio_map = parse_board(input);
    let rows = radio_map.len() as isize;
    let cols = radio_map[0].len() as isize;
    let coords_by_freq = get_coords_by_freq(&radio_map);
    println!("{:?}", coords_by_freq);
    let mut antinode_coords: HashSet<(isize, isize)> = HashSet::new();
    for ch in coords_by_freq.keys() {
        let coords = &coords_by_freq[ch];
        if coords.len() < 2 {
            continue;
        } // skip if there aren't 2 antennas
        coords
            .iter()
            .combinations(2)
            .map(|vec| (*vec[0], *vec[1]))
            .for_each(|((x1, y1), (x2, y2))| {
                let x_dist = (x2 as isize) - (x1 as isize);
                let y_dist = (y2 as isize) - (y1 as isize);
                antinode_coords.insert((x1, y1));
                antinode_coords.insert((x2, y2));

                let poss_bwd = (x1 - x_dist, y1 - y_dist);
                println!(
                    "[{}] {:?} delta {:?}",
                    ch,
                    [(x1, y1), (x2, y2)],
                    (x_dist, y_dist)
                );
                let mut poss_fwd = (x2 , y2 );
                loop {
                    poss_fwd = (poss_fwd.0 + x_dist, poss_fwd.1 + y_dist);
                    if (poss_fwd.0 < 0
                        || poss_fwd.1 < 0
                        || poss_fwd.0 >= cols
                        || poss_fwd.1 >= rows)
                    {
                        println!("[{}] {:?} FAILED", ch, poss_fwd);
                        break;
                    } else {
                        println!("[{}] {:?} PASSED", ch, poss_fwd);
                        antinode_coords.insert(poss_fwd);
                    };
                }
                let mut poss_bwd = (x1 , y1 );
                loop {
                    poss_bwd = (poss_bwd.0 - x_dist, poss_bwd.1 - y_dist);
                    if (poss_bwd.0 < 0
                        || poss_bwd.1 < 0
                        || poss_bwd.0 >= cols
                        || poss_bwd.1 >= rows)
                    {
                        println!("[{}] {:?} FAILED", ch, poss_bwd);
                        break;
                    } else {
                        println!("[{}] {:?} PASSED", ch, poss_bwd);
                        antinode_coords.insert(poss_bwd);
                    };
                }
            })
    }
    println!("{:?}", antinode_coords);
    draw_map(&radio_map, &antinode_coords);
    Some(antinode_coords.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
