use std::{iter, sync::LazyLock};

use rand::random_range;
use regex::Regex;

use crate::{cmd::check, state::State};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// # ParseResult
/// The [`parse`] function will take in a string argument and return one of these,
/// based on if the command was able to be executed.
#[derive(Clone)]
pub enum ParseResult {
    /// Exits the program.
    Exit,

    /// Continues execution, as normal.
    Cont(bool),

    /// Shows an invalid command, and the reason why.
    InvalidCmd(String),

    /// Shows an invalid argument, and the reason why.
    InvalidArgs(String),
}

/// Rolls any number of die, given standard dice notation.
///
/// To roll a dice in dndsh, type in:
/// ```text
/// roll <num_die>d<die_type> ... <num_die>d<die_type>
/// ```
/// - `num_die` is a [`usize`] which represents the number of dice to roll.
/// - `die_type` is a [`usize`] which represents the number of dice to roll.
///
/// Each dice roll will output the die rolled, the sum, the minimum, and the maximum.
fn roll(dice: Vec<&String>) -> ParseResult {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)d(\d+)").unwrap());

    for die in dice {
        if RE.is_match(die) {
            let caps = RE.captures(die).unwrap();

            let num_dice = caps.get(1).unwrap().as_str().parse::<usize>().unwrap_or(0);
            if num_dice == 0 {
                println!("CANNOT ROLL {}: Too many dice!", num_dice);
                continue;
            }
            let die_type = caps.get(2).unwrap().as_str().parse::<usize>().unwrap_or(0);

            if die_type == 0 {
                println!("CANNOT ROLL {}: Dice type is too much!", num_dice);
            }

            let (mut min, mut max, mut sum) = (0usize, 0usize, 0usize);

            let rolls: Vec<usize> = (0..num_dice)
                .map(|_| {
                    let n = random_range(1..=die_type);
                    min = if min != 0 { min.min(n) } else { n };
                    max = max.max(n);
                    sum += n;
                    n
                })
                .collect();

            print!("{} results:", die);
            if rolls.len() < 32 {
                rolls.iter().for_each(|n| print!(" {}", n));
            } else {
                rolls[0..5].iter().for_each(|n| print!(" {}", n));
                print!(" ... ({} more) ...", rolls.len() - 10);
                rolls[rolls.len() - 5..rolls.len()]
                    .iter()
                    .for_each(|n| print!(" {}", n));
            }

            println!("\n\tsum: {}", sum);
            println!("\tmin: {}", min);
            println!("\tmax: {}", max);
        } else {
            println!("INVALID DIE: {}", die)
        }
    }

    ParseResult::Cont(true)
}

/// Performs a skill/ability check in DND, which is done using a d20.
///
/// A check is perfomed using this syntax in dndsh:
/// ```text
/// check <MODIFIER>/<DC>
/// ```
/// where:
/// - `<MODIFIER>` is a positive/negative [`i128`], which specifies the modifier
///     the player has for that skill/ability.
/// - `<DC>` is a positive [`u128`], which specifies the difficulty class of the roll.
///
/// For more information on this,
/// see the [official guide](https://5thsrd.org/rules/abilities/ability_checks/).
fn check(arg: &str) -> ParseResult {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([\+\-]?\d+)/(\d+)").unwrap());

    if let Some(caps) = RE.captures(arg) {
        let modifier = caps.get(1).unwrap().as_str().parse::<i128>().unwrap();
        let dc = caps.get(2).unwrap().as_str().parse::<u128>().unwrap();

        let raw_roll = random_range(1..=20_i128);
        let roll = raw_roll + modifier;

        let roll_str = format!(
            "{} {} {} = {}/{}",
            raw_roll,
            if modifier >= 0 { '+' } else { '-' },
            modifier.abs(),
            roll,
            dc
        );

        let result = if roll < 0 || (roll as u128) < dc {
            "FAILED"
        } else {
            "PASSED"
        };

        println!("{}, check {}", roll_str, result);

        ParseResult::Cont(true)
    } else {
        ParseResult::InvalidArgs("check must have args in format <+-int>/<positive int>".to_owned())
    }
}

fn get_args(input: &str) -> Vec<String> {
    let s = input.trim();
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\S+)").unwrap());
    RE.captures_iter(s)
        .map(|cap| {
            let (_, [arg]) = cap.extract();
            arg.to_owned()
        })
        .collect()
}

fn do_last(s: &str, state: &mut State) -> ParseResult {
    let len = state.hist_len();
    if len == 0 {
        return ParseResult::InvalidCmd("No items in history".to_owned());
    }
    let mut idx = len - 1;
    if !s.is_empty() {
        match s.parse::<usize>() {
            Ok(n) => idx = n.saturating_sub(1),
            Err(_) => return ParseResult::InvalidArgs("Index must be a valid number".to_owned()),
        }
    }

    if let Some(cmd) = state.get_cmd(idx) {
        println!("{}", cmd);
        let res = parse(&cmd, state);
        match res {
            ParseResult::Cont(_) => ParseResult::Cont(false),
            _ => res,
        }
    } else {
        ParseResult::InvalidArgs(format!("Index is not in range [0, {}]", len))
    }
}

pub fn parse(input: &str, state: &mut State) -> ParseResult {
    let args = get_args(input);

    if args.is_empty() {
        ParseResult::Cont(false)
    } else {
        if args[0].starts_with('!') {
            return do_last(&args[0][1..], state);
        }
        match args[0].as_str() {
            // "" => ParseResult::Cont(false),
            "roll" => {
                if args.len() >= 2 {
                    roll(args[1..].iter().collect())
                } else {
                    ParseResult::InvalidArgs(
                        "Must specify at least one die in roll command".to_owned(),
                    )
                }
            }
            "check" => {
                if args.len() > 1 {
                    check::check(
                        &args[1],
                        if args.len() > 2 {
                            Some(args[1..].iter().collect())
                        } else {
                            None
                        },
                    )
                } else {
                    ParseResult::InvalidArgs("Must specify at least one check in check command".to_owned())
                }
            }
            "hist" | "history" => state.print_history(),
            "version" => {
                println!("dndsh v{}", VERSION);
                ParseResult::Cont(true)
            }
            "exit" | "quit" | "q" => {
                println!("Exited dndsh. Safe adventures, friend.");
                ParseResult::Exit
            }
            _ => ParseResult::InvalidCmd(args[0].clone()),
        }
    }
}
