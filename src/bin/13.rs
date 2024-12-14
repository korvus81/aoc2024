use regex::Regex;
use std::cmp::min;
use z3::ast::{Ast, Int};
use z3::*;

advent_of_code::solution!(13);

const OFFSET: u64 = 10000000000000u64;
const ALMOST_OFFSET: u64 = OFFSET - 1000;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut buttonA = "".to_string();
    let mut buttonB = "".to_string();
    let mut prize = "".to_string();
    let mut games: Vec<_> = vec![];
    let button_re = Regex::new(r"X[+]([0-9]+), Y[+]([0-9]+)").unwrap();
    let prize_re = Regex::new(r"X[=]([0-9]+), Y[=]([0-9]+)").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Button A:") {
            buttonA = line["Button A:".len()..].trim().to_string();
        } else if line.starts_with("Button B:") {
            buttonB = line["Button B:".len()..].trim().to_string();
        } else if line.starts_with("Prize: ") {
            prize = line["Prize: ".len()..].trim().to_string();

            let button_a_caps = button_re.captures(&buttonA).unwrap();
            let button_b_caps = button_re.captures(&buttonB).unwrap();
            let prize_caps = prize_re.captures(&prize).unwrap();
            games.push(Game {
                button_a: (
                    button_a_caps
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                    button_a_caps
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                ),
                button_b: (
                    button_b_caps
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                    button_b_caps
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                ),
                prize: (
                    prize_caps.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    prize_caps.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                ),
            });
        }
    }
    games
}

fn solve_game(game: Game) -> (u64, bool) {
    let max_a = min(
        game.prize.0 / game.button_a.0,
        game.prize.1 / game.button_a.1,
    );
    let max_b = min(
        game.prize.0 / game.button_b.0,
        game.prize.1 / game.button_b.1,
    );
    let mut best_a_presses = u64::MAX;
    let mut best_b_presses = u64::MAX;
    let mut best_cost = u64::MAX;
    for a_presses in 0..=max_a {
        let b_presses = min(
            (game.prize.0 - (a_presses * game.button_a.0)) / game.button_b.0,
            (game.prize.1 - (a_presses * game.button_a.1)) / game.button_b.1,
        );
        if (a_presses * game.button_a.0 + b_presses * game.button_b.0) == game.prize.0
            && (a_presses * game.button_a.1 + b_presses * game.button_b.1) == game.prize.1
        {
            let cost = (a_presses as u64 * 3) + b_presses as u64;
            if cost < best_cost {
                best_a_presses = a_presses;
                best_b_presses = b_presses;
                best_cost = cost;
            }
        }
    }
    if best_cost < u64::MAX {
        (best_cost, true)
    } else {
        (0, false)
    }
}

fn solve_game2(game: Game) -> (u64, bool) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let a_presses = Int::new_const(&ctx, "a_presses");
    let b_presses = Int::new_const(&ctx, "b_presses");
    let a_x_dist = Int::from_u64(&ctx, game.button_a.0);
    let a_y_dist = Int::from_u64(&ctx, game.button_a.1);
    let b_x_dist = Int::from_u64(&ctx, game.button_b.0);
    let b_y_dist = Int::from_u64(&ctx, game.button_b.1);
    let prize_x_dist = Int::from_u64(&ctx, game.prize.0);
    let prize_y_dist = Int::from_u64(&ctx, game.prize.1);

    let x_distance = Int::add(
        &ctx,
        &[
            &Int::mul(&ctx, &[&a_presses, &a_x_dist]),
            &Int::mul(&ctx, &[&b_presses, &b_x_dist]),
        ],
    );
    let y_distance = Int::add(
        &ctx,
        &[
            &Int::mul(&ctx, &[&a_presses, &a_y_dist]),
            &Int::mul(&ctx, &[&b_presses, &b_y_dist]),
        ],
    );
    solver.assert(&prize_x_dist._eq(&x_distance));
    solver.assert(&prize_y_dist._eq(&y_distance));
    let res = solver.check();
    if res == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let a_press = model.eval(&a_presses, true).unwrap().as_u64().unwrap();
        let b_press = model.eval(&b_presses, true).unwrap().as_u64().unwrap();
        let cost = a_press * 3 + b_press * 1;
        (cost, true)
    } else {
        (0, false)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let games = parse_games(input);
    println!("Games: {:?}", games);
    let mut total_cost = 0u64;
    for game in games {
        let (min_tokens, solved) = solve_game(game);
        if solved {
            total_cost += min_tokens;
        }
    }
    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let games = parse_games(input);
    println!("Games: {:?}", games);
    let mut total_cost = 0u64;
    for game in games {
        let altered_game = Game {
            prize: (game.prize.0 + OFFSET, game.prize.1 + OFFSET),
            ..game
        };
        let (min_tokens, solved) = solve_game2(altered_game);
        println!(
            "Game: {:?}, min_tokens: {}, solved: {}",
            game, min_tokens, solved
        );
        if solved {
            total_cost += min_tokens;
        }
    }
    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
