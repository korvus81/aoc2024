use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(4);

const XMAS_FWD: &'static str = "XMAS";
const XMAS_BKWD: &'static str = "SAMX";

fn get_range(input: & Vec<Vec<char>>, rows: Range<usize>, cols : Range<usize>) -> String {
    let mut s = String::new();
    for r in rows {
        for c in cols.clone() {
            s.push(input[r][c].clone());
        }
    }
    s
}

fn get_coords(input: & Vec<Vec<char>>, coords: &[(usize, usize)]) -> String {
    let mut s = String::new();
    for (r,c) in coords {
        s.push(input[*r][*c].clone());
    }
    s
}

pub fn part_one(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars().collect_vec()
        })
        .collect();

    let rows = tmp.len();
    let cols = tmp[0].len();

    let mut used: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    let mut matches = 0;

    for row in 0..rows {
        for col in 0..cols {
            let debug = (row == 3 && col == 6);
            if debug {
                println!("Row {}/{}, Col {}/{}",row,rows,col,cols);
            }
            if col <= (cols - 4) {
                let horiz = get_range(&tmp, row..row+1, col..col+4);
                if debug {
                    println!("[{},{}] Horiz: {:?}",row,col,horiz);
                }
                if horiz == XMAS_FWD { // forward
                    used[row][col] = tmp[row][col];
                    used[row][col+1] = tmp[row][col+1];
                    used[row][col+2] = tmp[row][col+2];
                    used[row][col+3] = tmp[row][col+3];
                    matches += 1;
                }
                if horiz == XMAS_BKWD { // backward
                    used[row][col] = tmp[row][col];
                    used[row][col+1] = tmp[row][col+1];
                    used[row][col+2] = tmp[row][col+2];
                    used[row][col+3] = tmp[row][col+3];
                    matches += 1;
                }
            }
            if row <= (rows - 4) {
                //println!("({:?},{:?}) of ({:?},{:?})\n{:?}\n",row,col, rows,cols,tmp);

                let vert = get_range(&tmp, row..row+4, col..col+1);
                if debug {
                    println!("[{},{}] Vert: {:?}",row,col,vert);
                }
                //println!("vert: {:?}",vert);
                if vert == XMAS_FWD {  // down
                    used[row][col] = tmp[row][col];
                    used[row+1][col] = tmp[row+1][col];
                    used[row+2][col] = tmp[row+2][col];
                    used[row+3][col] = tmp[row+3][col];
                    matches += 1;
                }
                if vert == XMAS_BKWD { // up
                    used[row][col] = tmp[row][col];
                    used[row+1][col] = tmp[row+1][col];
                    used[row+2][col] = tmp[row+2][col];
                    used[row+3][col] = tmp[row+3][col];
                    matches += 1;
                }
            }
            if col <= (cols - 4) && row <= (rows - 4) {
                let tl_to_br = get_coords(&tmp, &[(row,col),(row+1,col+1),(row+2,col+2),(row+3,col+3)]);
                if debug {
                    println!("[{},{}] TL_to_BR: {:?}",row,col,tl_to_br);
                }
                if tl_to_br == XMAS_FWD { // diag down-right
                    used[row][col] = tmp[row][col];
                    used[row+1][col+1] = tmp[row+1][col+1];
                    used[row+2][col+2] = tmp[row+2][col+2];
                    used[row+3][col+3] = tmp[row+3][col+3];
                    matches += 1;
                }
                if tl_to_br == XMAS_BKWD { // diag up-left
                    used[row][col] = tmp[row][col];
                    used[row+1][col+1] = tmp[row+1][col+1];
                    used[row+2][col+2] = tmp[row+2][col+2];
                    used[row+3][col+3] = tmp[row+3][col+3];
                    matches += 1;
                }
                let bl_to_tr = get_coords(&tmp, &[(row+3,col),(row+2,col+1),(row+1,col+2),(row,col+3)]);
                if debug {
                    println!("[{},{}] BL_to_TR: {:?}",row,col,bl_to_tr);
                }
                if bl_to_tr == XMAS_FWD { // diag up-right
                    used[row+3][col] = tmp[row+3][col];
                    used[row+2][col+1] = tmp[row+2][col+1];
                    used[row+1][col+2] = tmp[row+1][col+2];
                    used[row][col+3] = tmp[row][col+3];
                    matches += 1;
                }
                if bl_to_tr == XMAS_BKWD { // diag down-left
                    used[row+3][col] = tmp[row+3][col];
                    used[row+2][col+1] = tmp[row+2][col+1];
                    used[row+1][col+2] = tmp[row+1][col+2];
                    used[row][col+3] = tmp[row][col+3];
                    matches += 1;
                }
            }
        }
    }
    println!("");
    println!("");
    for row in tmp {
        print!(" ");
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!("");
    println!("");
    for row in used {
        print!(" ");
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    Some(matches)

}

const MAS_FWD: &'static str = "MAS";
const MAS_BKWD: &'static str = "SAM";

fn check_x(input: & Vec<Vec<char>>, row:usize,col:usize) -> bool {
    println!("Checking x at row {}, col {}", row, col);
    let tl_to_br = get_coords(input, &[(row,col),(row+1,col+1),(row+2,col+2)]);
    let tr_to_bl = get_coords(input, &[(row,col+2),(row+1,col+1),(row+2,col)]);
    (tl_to_br == MAS_FWD || tl_to_br == MAS_BKWD) && (tr_to_bl == MAS_FWD || tr_to_bl == MAS_BKWD)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tmp: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars().collect_vec()
        })
        .collect();

    let rows = tmp.len();
    let cols = tmp[0].len();

    let mut used: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    let mut matches = 0;

    for row in 0..rows-2 {
        for col in 0..cols-2 {
            if check_x(&tmp, row,col) {
                used[row][col] = tmp[row][col];
                used[row+1][col+1] = tmp[row+1][col+1];
                used[row+2][col+2] = tmp[row+2][col+2];
                used[row+2][col] = tmp[row+2][col];
                used[row][col+2] = tmp[row][col+2];
                matches += 1;
            }
        }
    }
    println!("");
    println!("");
    for row in tmp {
        print!(" ");
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!("");
    println!("");
    for row in used {
        print!(" ");
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    Some(matches)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
