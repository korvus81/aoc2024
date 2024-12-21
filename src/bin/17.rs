// part 2 was a disaster.  See d17.py in the root for part of what I used to get the result.

use itertools::Itertools;
use std::cmp::min;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

advent_of_code::solution!(17);

/*
Register A: 46187030
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0
 */

fn parse_input(s: &str) -> (u64, u64, u64, Vec<u64>) {
    let lines = s.trim().lines().collect_vec();
    let regA: u64 = lines[0]
        .split("A:")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let regB: u64 = lines[1]
        .split("B:")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let regC: u64 = lines[2]
        .split("C:")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let program: Vec<u64> = lines[4]
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    (regA, regB, regC, program)
}

fn get_operand(regA: &mut u64, regB: &mut u64, regC: &mut u64, op: u64, operand_raw: u64) -> u64 {
    let operand = match op {
        //combo ops
        0 | 2 | 5 | 6 | 7 => match operand_raw {
            0 | 1 | 2 | 3 => operand_raw,
            4 => *regA,
            5 => *regB,
            6 => *regC,
            7 => unreachable!(),
            _ => unreachable!(),
        },
        // literal ops
        1 | 3 | 4 => operand_raw,
        _ => unreachable!(),
    };
    operand
}

fn run_program(
    regA_init: u64,
    regB_init: u64,
    regC_init: u64,
    program: &Vec<u64>,
) -> (u64, u64, u64, Vec<u64>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut regA = regA_init;
    let mut regB = regB_init;
    let mut regC = regC_init;
    let mut ip: u64 = 0;
    let mut output: Vec<u64> = Vec::new();
    let op_name: Vec<&str> = vec!["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];
    while ip < program.len() as u64 {
        let mut nextip = ip + 2;
        let op = program[ip as usize];
        let operand_raw = program[ip as usize + 1];
        let operand = get_operand(&mut regA, &mut regB, &mut regC, op, operand_raw);
        // writeln!(
        //     &mut stdout,
        //     "INITIAL: Op:{}({}) Operand:{} (raw:{}) || A:{}, B:{}, C:{}, Out: {:?}",
        //     op, op_name[op as usize], operand, operand_raw, regA, regB, regC, output
        // )
        //     .unwrap();
        match op {
            // adv
            0 => regA = regA / 2u64.pow(operand as u32),
            // bxl
            1 => regB = regB ^ operand,
            // bst
            2 => regB = operand % 8,
            // jnz
            3 => {
                if regA != 0 {
                    nextip = operand;
                }
            }
            // bxc
            4 => regB = regB ^ regC, // ignores operand
            // out
            5 => output.push(operand % 8),
            // bdv
            6 => regB = regA / 2u64.pow(operand as u32),
            // cdv
            7 => regC = regA / 2u64.pow(operand as u32),
            _ => unreachable!(),
        };
        // writeln!(
        //     &mut stdout,
        //     "AFTER: Op:{}({}) Operand:{} (raw:{}) || A:{}, B:{}, C:{}, Out: {:?}",
        //     op, op_name[op as usize], operand, operand_raw, regA, regB, regC, output
        // )
        // .unwrap();
        // for i in 0..program.len() {
        //     if i == ip as usize {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", i).unwrap();
        //         stdout.reset().unwrap();
        //     } else if i == ip as usize + 1 {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", i).unwrap();
        //         stdout.reset().unwrap();
        //     } else {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", i).unwrap();
        //         stdout.reset().unwrap();
        //     }
        // }
        // writeln!(&mut stdout, "").unwrap();
        ip = nextip;
    }
    (regA, regB, regC, output)
}

pub fn part_one(input: &str) -> Option<String> {
    let (regA_init, regB_init, regC_init, program) = parse_input(input.clone());
    println!("A: {}", regA_init);
    println!("B: {}", regB_init);
    println!("C: {}", regC_init);
    println!("Program: {:?}", program);
    let (a, b, c, output) = run_program(regA_init, regB_init, regC_init, &program);

    // wrong: 4,6,3,7,1,7,7,4,6
    Some(output.iter().join(","))
}

fn run_program2(regA_init: u64, regB_init: u64, regC_init: u64, program: &Vec<u64>) -> bool {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut regA = regA_init;
    let mut regB = regB_init;
    let mut regC = regC_init;
    let mut ip: u64 = 0;
    let mut output: Vec<u64> = Vec::new();
    let op_name: Vec<&str> = vec!["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];
    while ip < program.len() as u64 {
        let mut nextip = ip + 2;
        let op = program[ip as usize];
        let operand_raw = program[ip as usize + 1];
        let operand = get_operand(&mut regA, &mut regB, &mut regC, op, operand_raw);
        // writeln!(
        //     &mut stdout,
        //     "INITIAL: Op:{}({}) Operand:{} (raw:{}) || A:{}, B:{}, C:{}, Out: {:?}",
        //     op, op_name[op as usize], operand, operand_raw, regA, regB, regC, output
        // )
        //     .unwrap();
        match op {
            // adv
            0 => regA = regA / 2u64.checked_pow(operand as u32).unwrap(),
            // bxl
            1 => regB = regB ^ operand,
            // bst
            2 => regB = operand % 8,
            // jnz
            3 => {
                if regA != 0 {
                    nextip = operand;
                }
            }
            // bxc
            4 => regB = regB ^ regC, // ignores operand
            // out
            5 => {
                let outval = operand % 8;
                output.push(outval);
                // if output.len() < program.len() && program[output.len()] == outval {
                //     if output.len() > 9 {
                //         println!("a_init:{} out[{}]:{}", regA_init, output.len(), outval);
                //     }
                //     output.push(operand % 8);
                // } else {
                //     if output.len() > 10 {
                //         println!("a_init:{} out[{}]:{} XXX != {}", regA_init, output.len(), outval,program[output.len()]);
                //     }
                //     return false;
                // }
            }
            // bdv
            6 => regB = regA / 2u64.checked_pow(operand as u32).unwrap(),
            // cdv
            7 => regC = regA / 2u64.checked_pow(operand as u32).unwrap(),
            _ => unreachable!(),
        };
        // writeln!(
        //     &mut stdout,
        //     "AFTER: Op:{}({}) Operand:{} (raw:{}) || A:{}, B:{}, C:{}, Out: {:?}",
        //     op, op_name[op as usize], operand, operand_raw, regA, regB, regC, output
        // )
        //     .unwrap();
        // for i in 0..program.len() {
        //     if i == ip as usize {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", program[i]).unwrap();
        //         stdout.reset().unwrap();
        //     } else if i == ip as usize + 1 {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", program[i]).unwrap();
        //         stdout.reset().unwrap();
        //     } else {
        //         stdout
        //             .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        //             .unwrap();
        //         write!(&mut stdout, "{} ", program[i]).unwrap();
        //         stdout.reset().unwrap();
        //     }
        // }
        // writeln!(&mut stdout, "").unwrap();
        ip = nextip;
    }
    println!(
        "=== Initial A: {} -> (rev) {:?} -> {:?}",
        regA_init,
        output
            .iter()
            .rev()
            .map(|x| (*x).to_string())
            .collect_vec()
            .join(""),
        output
    );
    //sleep(Duration::from_millis(1));
    if output.len() == program.len() {
        for i in 0..program.len() {
            if output[i] != program[i] {
                return false;
            }
        }
        //if output.iter().zip(&program.iter()).all(|(a, b)| a == b) {
        return true;
    }
    false
}

fn optimized(init_a: u64) -> Vec<u64> {
    let mut regA = init_a;
    let mut regB = 0;
    let mut regC = 0;
    let mut out: Vec<u64> = Vec::new();
    loop {
        regB = regA % 8;
        regB = regB ^ 5;

        regC = regA >> regB; //regA / 2u64.pow(regB as u32);

        regA = regA / 8;
        regB = regB ^ regC;
        regB = regB ^ 6;
        out.push(regB % 8);
        if regA == 0 {
            break;
        };
    }
    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let (regA_init, regB_init, regC_init, program) = parse_input(input.clone());
    println!("A: {}", regA_init);
    println!("B: {}", regB_init);
    println!("C: {}", regC_init);
    println!("Program: {:?}", program);
    // up to 1002690000000
    //       8589934592
    //       68719476736
    let mut a_init = 0o3000000000000000;
    //let mut cnt = 0o3066523703646271;//1<<6;
    let mut cnt = 0; //0o3000000000000000;//1<<6;
    let mut high_bits = 0u64;
    let mut digit = 0;
    let mut digits: Vec<u64> = vec![0; 16];

    loop {
        //a_init = 3+(cnt>>3);//3 + (5 << 3) + (0 << 6) + (4 << 9) + (6 << 12) + (4 << 15) + (1 << 18) + (2 << 21) + (5 << 24) + (1 << 27) + (0 << 30) + (7 << 33) + (4 << 36) + (4 << 39) + (cnt << 41);// + (cnt << 36);
        // println!(
        //     "A: {:#o} cnt:{} digit:{} digits:{:?}",
        //     a_init, cnt, digit, digits
        // );
        a_init = 0;
        for (digit_num, digit_value) in digits.iter().enumerate() {
            a_init = a_init << 3;
            a_init = a_init + digit_value;
        }
        //a_init = (a_init & ((2u64.pow(49)-1) ^ (0x7<<(digit*3)))) + (high_bits << 48);
        //a_init = a_init | (cnt << (digit*3));
        println!(
            "A: {:#o} cnt:{} digit:{} digits:{:?} (after)",
            a_init, cnt, digit, digits
        );
        let output = optimized(a_init);
        println!(
            "=== OPT Initial A: {} ({:#o}) Output: {:?} (program: {:?}",
            a_init, a_init, output, program
        );

        let mut matched = false;
        if output.len() == program.len() {
            println!(
                "{} =? {}",
                output[output.len() - 1 - digit],
                program[program.len() - 1 - digit]
            );
            if output[output.len() - 1 - digit] == program[program.len() - 1 - digit] {
                matched = true;
                println!("incrementing digit");
                digit = digit + 1;
                cnt = 0;
            }
        }
        if !matched {
            digits[digit] = (digits[digit] + 1) % 8;
        }
    }
    // 109019476330651 from python code with bit manipulation
    // 35519602255915
    // 46514718533675 is too low
    // 109309586263225 is too high
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_inputs() {
        {
            let (a, b, c, output) = run_program(0, 0, 9, &vec![2, 6]);
            assert_eq!(b, 1);
        }
        {
            let (a, b, c, output) = run_program(10, 0, 0, &vec![5, 0, 5, 1, 5, 4]);
            assert_eq!(output, vec![0, 1, 2]);
        }
        {
            let (a, b, c, output) = run_program(2024, 0, 0, &vec![0, 1, 5, 4, 3, 0]);
            assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        }
        {
            let (a, b, c, output) = run_program(0, 29, 0, &vec![1, 7]);
            assert_eq!(b, 26);
        }
        {
            let (a, b, c, output) = run_program(0, 2024, 43690, &vec![4, 0]);
            assert_eq!(b, 44354);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
