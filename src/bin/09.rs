use std::iter::repeat_n;

advent_of_code::solution!(9);

fn input_to_blocks(input: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::<Option<usize>>::new();
    for (i, ch) in input.trim().chars().enumerate() {
        let isFile = i % 2 == 0;
        let val = ch.to_digit(10).unwrap();
        for v in 0..val {
            if !isFile {
                blocks.push(None);
            } else {
                let filenum = i / 2;
                blocks.push(Some(filenum));
            }
        }
    }
    blocks
}

fn display_blocks(blocks: &Vec<Option<usize>>) {
    println!(
        "{:?}",
        blocks
            .iter()
            .map(|x| match x {
                None => ".".to_string(),
                Some(n) => n.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    );
}

fn calc_checksum(blocks: &Vec<Option<usize>>) -> u64 {
    let checksum = blocks
        .iter()
        .enumerate()
        .map(|(i, x)| {
            i as u64
                * match x {
                    None => 0,
                    Some(n) => *n,
                } as u64
        })
        .sum();
    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = input_to_blocks(input);
    display_blocks(&blocks);
    let blocks_size = blocks.len();
    let mut last_file_block = blocks_size - 1;
    while blocks[last_file_block] == None && last_file_block > 0 {
        last_file_block -= 1;
    }
    let mut i: usize = 0;
    while i < last_file_block {
        if blocks[i] == None {
            blocks[i] = blocks[last_file_block];
            blocks[last_file_block] = None;
            while blocks[last_file_block] == None && last_file_block > 0 {
                last_file_block -= 1;
            }
        }
        i += 1;
    }
    display_blocks(&blocks);
    let checksum = calc_checksum(&blocks);
    return Some(checksum);
}

fn input_to_sparse_blocks(input: &str) -> Vec<(Option<usize>, u32)> {
    let mut blocks = Vec::<(Option<usize>, u32)>::new();
    for (i, ch) in input.trim().chars().enumerate() {
        let isFile = i % 2 == 0;
        let val = ch.to_digit(10).unwrap();
        if isFile {
            let filenum = i / 2;
            blocks.push((Some(filenum), val));
        } else {
            blocks.push((None, val));
        }
    }
    blocks
}

fn display_sparse_blocks(blocks: &Vec<(Option<usize>, u32)>) {
    for b in blocks {
        match b {
            (None, l) => {
                for _ in 0..*l {
                    print!(".")
                }
            }
            (Some(n), l) => {
                for _ in 0..*l {
                    print!("{}", n)
                }
            }
        }
    }
    println!("");
}

fn display_sparse_blocks2(blocks: &Vec<(Option<usize>, u32)>) {
    for b in blocks {
        match b {
            (None, l) => print!("(.x{}) ", l),
            (Some(n), l) => print!("({}x{}) ", n, l),
        }
    }
    println!("");
}

fn calc_checksum_sparse(blocks: &Vec<(Option<usize>, u32)>) -> u64 {
    let checksum = blocks
        .iter()
        .flat_map(|(x, blocklen)| {
            repeat_n(
                match x {
                    None => 0_u32,
                    Some(n) => *n as u32,
                },
                *blocklen as usize,
            )
        })
        .enumerate()
        .map(|(i, x)| i as u64 * x as u64)
        .sum();
    checksum
}

fn collapse_sparse_blocks(blocks: Vec<(Option<usize>, u32)>) -> Vec<(Option<usize>, u32)> {
    let mut new_blocks = Vec::<(Option<usize>, u32)>::new();
    let mut emptylen = 0;
    for b in blocks {
        if b.0 == None {
            emptylen += b.1;
        } else {
            // because I only push if I see a file, this should remove empties from the end as well as coalescing emptys
            if emptylen > 0 {
                new_blocks.push((None, emptylen));
                emptylen = 0;
            }
            new_blocks.push(b.clone());
        }
    }
    new_blocks
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = input_to_sparse_blocks(input);

    //display_sparse_blocks(&blocks);
    blocks = collapse_sparse_blocks(blocks);
    let files = blocks
        .iter()
        .filter(|x| x.0 != None)
        .rev()
        .map(|(x, n)| (x.unwrap(), *n))
        .collect::<Vec<_>>();
    for f in &files {
        println!("{:?}", f);
    }
    for (fnum, fileblocklen) in files {
        //for (i,(blk,blklen)) in &blocks.iter().enumerate() {
        for i in 0..blocks.len() {
            let (blk, blklen) = blocks[i];
            if blk == Some(fnum) {
                break;
            }; // nowhere to move
            if blk.is_none() {
                // empty space
                if fileblocklen <= blklen {
                    // remove the existing entry for this block
                    blocks = blocks
                        .iter()
                        //.filter(|x| x.0 != Some(fnum))
                        .map(|x| if x.0 == Some(fnum) { (None, x.1) } else { *x })
                        .collect();
                    // put it in place of the empty space
                    blocks[i] = (Some(fnum), fileblocklen);
                    if (blklen - fileblocklen > 0) {
                        // insert remaining empty space
                        blocks.insert(i + 1, (None, blklen - fileblocklen));
                    }
                    break;
                }
            }
        }
        blocks = collapse_sparse_blocks(blocks);
        //display_sparse_blocks(&blocks);
    }

    //display_sparse_blocks(&blocks);
    let checksum = calc_checksum_sparse(&blocks);
    return Some(checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
