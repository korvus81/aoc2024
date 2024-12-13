use itertools::Itertools;
//use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
//use std::rc::Rc;
use disjoint::{DisjointSet, DisjointSetVec};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Region {
    coords: HashSet<(i32, i32)>,
}

impl Region {
    fn new() -> Region {
        Region {
            coords: HashSet::new(),
        }
    }

    fn add(&mut self, coord: (i32, i32)) {
        self.coords.insert(coord);
    }

    fn area(&self) -> u64 {
        self.coords.len() as u64
    }

    fn perimeter(&self) -> u64 {
        let mut perimeter = 0;
        for (x, y) in self.coords.iter() {
            let mut exposed_sides = 0u64;
            if !self.coords.contains(&(*x - 1, *y)) {
                exposed_sides += 1;
            }
            if !self.coords.contains(&(*x + 1, *y)) {
                exposed_sides += 1;
            }
            if !self.coords.contains(&(*x, *y - 1)) {
                exposed_sides += 1;
            }
            if !self.coords.contains(&(*x, *y + 1)) {
                exposed_sides += 1;
            }
            if self.coords.len() > 1 {
                assert_ne!(exposed_sides, 4); // this would imply we aren't connected...ignoring if it is a region of 1
            }
            perimeter += exposed_sides;
        }
        perimeter
    }

    fn cost(&self) -> u64 {
        self.area() * self.perimeter()
    }

    fn sides(&self) -> u64 {
        let mut sides_count = 0;
        let mut exposed_l: HashSet<(i32, i32)> = HashSet::new();
        let mut exposed_r: HashSet<(i32, i32)> = HashSet::new();
        let mut exposed_u: HashSet<(i32, i32)> = HashSet::new();
        let mut exposed_d: HashSet<(i32, i32)> = HashSet::new();

        for (x, y) in self.coords.iter() {
            //let mut exposed_sides = 0u64;
            if !self.coords.contains(&(*x - 1, *y)) {
                exposed_l.insert((*x, *y));
            }
            if !self.coords.contains(&(*x + 1, *y)) {
                exposed_r.insert((*x, *y));
            }
            if !self.coords.contains(&(*x, *y - 1)) {
                exposed_u.insert((*x, *y));
            }
            if !self.coords.contains(&(*x, *y + 1)) {
                exposed_d.insert((*x, *y));
            }
        }
        for xset in [exposed_l, exposed_r] {
            let xvals = xset
                .iter()
                .map(|(x, _y)| *x)
                .sorted()
                .unique()
                .collect_vec();
            for xv in xvals {
                let yvals = xset
                    .iter()
                    .filter(|(x, _y)| *x == xv)
                    .map(|(_x, y)| *y)
                    .sorted()
                    .unique()
                    .collect_vec();
                if yvals.len() == 0 {
                    continue;
                }
                let mut lastyv = -100;
                for yv in yvals {
                    if yv.abs_diff(lastyv) == 1 {
                        lastyv = yv;
                        continue; // don't count
                    } else {
                        lastyv = yv;
                        sides_count += 1; // disjoint at same X, so count it
                    }
                }
            }
        }
        for yset in [exposed_u, exposed_d] {
            let yvals = yset
                .iter()
                .map(|(_x, y)| *y)
                .sorted()
                .unique()
                .collect_vec();
            for yv in yvals {
                let xvals = yset
                    .iter()
                    .filter(|(_x, y)| *y == yv)
                    .map(|(x, _y)| *x)
                    .sorted()
                    .unique()
                    .collect_vec();
                if xvals.len() == 0 {
                    continue;
                }
                let mut lastxv = -100;
                for xv in xvals {
                    if xv.abs_diff(lastxv) == 1 {
                        lastxv = xv;
                        continue; // don't count
                    } else {
                        lastxv = xv;
                        sides_count += 1; // disjoint at same X, so count it
                    }
                }
            }
        }
        sides_count
    }
    fn bulkcost(&self) -> u64 {
        self.area() * self.sides()
    }
}

impl Hash for Region {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords.iter().for_each(|(x, y)| {
            x.hash(state);
            y.hash(state);
        });
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let garden: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();
    let mut coords_to_region: HashMap<(i32, i32), usize> = HashMap::new();
    let mut all_regions: Vec<Region> = vec![];
    let mut val_to_coords: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            let val = garden[y][x];
            //let mut maybe_region: Option<&mut Region> = None;;
            let xval = x as i32;
            let yval = y as i32;
            val_to_coords
                .entry(val)
                .or_insert_with(HashSet::new)
                .insert((xval, yval));
        }
    }
    for (val, coords) in val_to_coords.iter() {
        let mut dset: DisjointSetVec<(i32, i32)> = DisjointSetVec::with_capacity(coords.len());
        for (x, y) in coords.iter() {
            dset.push((*x, *y));
        }

        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..dset.len() - 1 {
                for j in i + 1..dset.len() {
                    // if they aren't joined in a set, and they
                    if !dset.is_joined(i, j)
                        && (dset[i].0.abs_diff(dset[j].0) + dset[i].1.abs_diff(dset[j].1)) == 1
                    {
                        dset.join(i, j);
                        changed = true;
                    }
                }
            }
        }

        println!("Post-merges {:?} -> sets: {:?}", val, dset);
        for s in dset {
            let mut r = Region::new();
            for (x, y) in s.iter() {
                r.add((*x, *y));
            }
            all_regions.push(r);
        }
    }

    let mut total_cost = 0u64;
    for region in all_regions.iter() {
        println!("Region {:?}: {}", region, region.cost());
        total_cost += region.cost();
    }
    Some(total_cost)
}

pub fn part_one_old(input: &str) -> Option<u64> {
    let garden: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();
    let mut coords_to_region: HashMap<(i32, i32), usize> = HashMap::new();
    let mut all_regions: Vec<Region> = vec![];
    let mut val_to_coords: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            let val = garden[y][x];
            //let mut maybe_region: Option<&mut Region> = None;;
            let xval = x as i32;
            let yval = y as i32;
            val_to_coords
                .entry(val)
                .or_insert_with(HashSet::new)
                .insert((xval, yval));
        }
    }
    for (val, coords) in val_to_coords.iter() {
        let mut sets: Vec<HashSet<(i32, i32)>> = vec![];
        for (x, y) in coords.iter() {
            if sets.is_empty() {
                sets.push(HashSet::from([(*x, *y)]));
            } else {
                let mut added = false;
                for i in 0..sets.len() {
                    if !added
                        && (sets[i].contains(&(*x, *y))
                            || sets[i].contains(&(*x + 1, *y))
                            || sets[i].contains(&(*x - 1, *y))
                            || sets[i].contains(&(*x, *y + 1))
                            || sets[i].contains(&(*x, *y - 1)))
                    {
                        // there is an adjacent point (or itself, just in case) in this set, so we belong there
                        sets[i].insert((*x, *y));
                        added = true;
                        break;
                    }
                }
                if !added {
                    sets.push(HashSet::from([(*x, *y)])); // add to new set
                }
            }
        }

        //println!("Pre-merge {:?} -> sets: {:?}", val, sets);
        let mut changed = true;
        while changed {
            changed = false;
            let mut to_merge: Option<(usize, usize)> = None;
            for i in 0..sets.len() - 1 {
                for j in i + 1..sets.len() {
                    for (x1, y1) in sets[i].iter() {
                        for (x2, y2) in sets[j].iter() {
                            // to_merge.is_none() is because I have no idea how to break all the way out
                            if to_merge.is_none()
                                && (((*x1 == *x2) && (*y1 <= (*y2) + 1 && *y1 >= (*y2) - 1))
                                    || ((*y1 == *y2) && (*x1 <= (*x2) + 1 && *x1 >= (*x2) - 1)))
                            {
                                // these sets match
                                //println!("  Match ({:?}) is adjacent to ({:?})", sets[i], sets[j]);
                                to_merge = Some((i, j));
                                break;
                            }
                        }
                    }
                }
            }
            if to_merge.is_some() {
                let (first_set, second_set) = to_merge.unwrap();
                //println!("  Merging {:?} + {:?}", sets[first_set], sets[second_set]);
                changed = true;
                let mut newsets: Vec<HashSet<(i32, i32)>> = vec![];
                for i in 0..sets.len() {
                    if i == first_set {
                        let newset: HashSet<(i32, i32)> = HashSet::from_iter(
                            sets[i]
                                .clone()
                                .union(&sets[second_set])
                                .into_iter()
                                .map(|x| *x),
                        );
                        newsets.push(newset);
                    } else if i == second_set {
                        // skip because we already unioned it into the first
                    } else {
                        newsets.push(sets[i].clone());
                    }
                }
                sets = newsets;
                //println!("   Interim {:?} -> sets: {:?}", val, sets);
            }
        }
        println!("Post-merges {:?} -> sets: {:?}", val, sets);
        for s in sets.iter() {
            let mut r = Region::new();
            for (x, y) in s.iter() {
                r.add((*x, *y));
            }
            all_regions.push(r);
        }
    }

    let mut total_cost = 0u64;
    for region in all_regions.iter() {
        println!("Region {:?}: {}", region, region.cost());
        total_cost += region.cost();
    }
    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let garden: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();
    let mut coords_to_region: HashMap<(i32, i32), usize> = HashMap::new();
    let mut all_regions: Vec<Region> = vec![];
    let mut val_to_coords: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            let val = garden[y][x];
            //let mut maybe_region: Option<&mut Region> = None;;
            let xval = x as i32;
            let yval = y as i32;
            val_to_coords
                .entry(val)
                .or_insert_with(HashSet::new)
                .insert((xval, yval));
        }
    }
    for (val, coords) in val_to_coords.iter() {
        let mut sets: Vec<HashSet<(i32, i32)>> = vec![];
        for (x, y) in coords.iter() {
            if sets.is_empty() {
                sets.push(HashSet::from([(*x, *y)]));
            } else {
                let mut added = false;
                for i in 0..sets.len() {
                    if !added
                        && (sets[i].contains(&(*x, *y))
                            || sets[i].contains(&(*x + 1, *y))
                            || sets[i].contains(&(*x - 1, *y))
                            || sets[i].contains(&(*x, *y + 1))
                            || sets[i].contains(&(*x, *y - 1)))
                    {
                        // there is an adjacent point (or itself, just in case) in this set, so we belong there
                        sets[i].insert((*x, *y));
                        added = true;
                        break;
                    }
                }
                if !added {
                    sets.push(HashSet::from([(*x, *y)])); // add to new set
                }
            }
        }

        //println!("Pre-merge {:?} -> sets: {:?}", val, sets);
        let mut changed = true;
        while changed {
            changed = false;
            let mut to_merge: Option<(usize, usize)> = None;
            for i in 0..sets.len() - 1 {
                for j in i + 1..sets.len() {
                    for (x1, y1) in sets[i].iter() {
                        for (x2, y2) in sets[j].iter() {
                            // to_merge.is_none() is because I have no idea how to break all the way out
                            if to_merge.is_none()
                                && (((*x1 == *x2) && (*y1 <= (*y2) + 1 && *y1 >= (*y2) - 1))
                                    || ((*y1 == *y2) && (*x1 <= (*x2) + 1 && *x1 >= (*x2) - 1)))
                            {
                                // these sets match
                                //println!("  Match ({:?}) is adjacent to ({:?})", sets[i], sets[j]);
                                to_merge = Some((i, j));
                                break;
                            }
                        }
                    }
                }
            }
            if to_merge.is_some() {
                let (first_set, second_set) = to_merge.unwrap();
                //println!("  Merging {:?} + {:?}", sets[first_set], sets[second_set]);
                changed = true;
                let mut newsets: Vec<HashSet<(i32, i32)>> = vec![];
                for i in 0..sets.len() {
                    if i == first_set {
                        let newset: HashSet<(i32, i32)> = HashSet::from_iter(
                            sets[i]
                                .clone()
                                .union(&sets[second_set])
                                .into_iter()
                                .map(|x| *x),
                        );
                        newsets.push(newset);
                    } else if i == second_set {
                        // skip because we already unioned it into the first
                    } else {
                        newsets.push(sets[i].clone());
                    }
                }
                sets = newsets;
                //println!("   Interim {:?} -> sets: {:?}", val, sets);
            }
        }
        println!("Post-merges {:?} -> sets: {:?}", val, sets);
        for s in sets.iter() {
            let mut r = Region::new();
            for (x, y) in s.iter() {
                r.add((*x, *y));
            }
            all_regions.push(r);
        }
    }

    let mut total_cost = 0u64;
    for region in all_regions.iter() {
        println!("Region {:?}: {}", region, region.bulkcost());
        total_cost += region.bulkcost();
    }
    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region() {
        let mut region = Region::new();
        for x in 0..5 {
            for y in 0..5 {
                if !((y == 1 || y == 3) && (x == 1 || x == 3)) {
                    region.add((x, y));
                }
            }
        }
        assert_eq!(region.perimeter(), 36);
        assert_eq!(region.area(), 21);
        assert_eq!(region.cost(), 756);
    }
    #[test]
    fn test_region_bulkcost() {
        let mut region = Region::new();
        region.add((2, 1));
        region.add((2, 2));
        region.add((3, 2));
        region.add((3, 3));
        assert_eq!(region.perimeter(), 10);
        assert_eq!(region.sides(), 8);
        assert_eq!(region.area(), 4);
        assert_eq!(region.bulkcost(), 32);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
