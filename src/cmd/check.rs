use crate::parser::ParseResult;
use rand::random_range;
use regex::Regex;
use std::sync::LazyLock;

enum CheckType {
    Normal,
    Advantage,
    Disadvantage,
}

/// Performs a skill/ability check in DND, which is done using a d20.
///
/// A check is perfomed using this syntax in dndsh:
/// ```text
/// check <MODIFIER>/<DC> [options]
/// ```
/// where:
/// - `<MODIFIER>` is a positive/negative [`i128`], which specifies the modifier
///     the player has for that skill/ability.
/// - `<DC>` is a positive [`u128`], which specifies the difficulty class of the roll.
/// - `[options]` are some flags, which can be:
///     - `-a`, which means to roll with advantage. This means that 2d20 are rolled, and
///     the one with the higher roll is counted.
///     - `-d`, which means to roll with disadvantage. This means that 2d20 are rolled, and
///     the one with the lower roll is counted.
///
/// For more information on this,
/// see the [official guide](https://5thsrd.org/rules/abilities/ability_checks/).
pub fn check(arg: &str, opts: Option<Vec<&String>>) -> ParseResult {
    println!("{}", arg);
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([\+\-]?\d+)/(\d+)").unwrap());

    if let Some(caps) = RE.captures(arg) {
        let modifier = caps.get(1).unwrap().as_str().parse::<i128>().unwrap();
        let dc = caps.get(2).unwrap().as_str().parse::<u128>().unwrap();
        let mut check_type = CheckType::Normal;

        if let Some(opts) = opts{
            for opt in opts {
                match opt.as_str() {
                    "-a" => check_type = CheckType::Advantage,
                    "-d" => check_type = CheckType::Disadvantage,
                    _ => (),
                }
            }
        }
        check_roll(modifier, dc, check_type);

        ParseResult::Cont(true)
    } else {
        ParseResult::InvalidArgs(
            "check must have argument in format <+-int>/<positive int>".to_owned(),
        )
    }
}

fn check_roll(modifier: i128, dc: u128, check_type: CheckType) {
    let raw_roll = random_range(1..=20_i128);
    let other_roll = if matches!(check_type, CheckType::Normal) {
        0
    } else {
        random_range(1..=20_i128)
    };

    let roll = match check_type {
        CheckType::Normal => raw_roll + modifier,
        CheckType::Advantage => raw_roll.max(other_roll) + modifier,
        CheckType::Disadvantage => raw_roll.min(other_roll) + modifier,
    };

    let roll_calc_str = match check_type {
        CheckType::Normal => format!(
            "{} {} {} = {}",
            roll,
            if modifier >= 0 { '+' } else { '-' },
            modifier.abs(),
            roll
        ),
        CheckType::Advantage => format!(
            "({} > {}) {} {} = {}",
            raw_roll.max(other_roll),
            raw_roll.min(other_roll),
            if modifier >= 0 { '+' } else { '-' },
            modifier.abs(),
            roll
        ),
        CheckType::Disadvantage => format!(
            "({} < {}) {} {} = {}",
            raw_roll.min(other_roll),
            raw_roll.max(other_roll),
            if modifier >= 0 { '+' } else { '-' },
            modifier.abs(),
            roll
        ),
    };

    let roll_str = format!("{}/{}", roll_calc_str, dc);

    let result = if roll < 0 || (roll as u128) < dc {
        "FAILED"
    } else {
        "PASSED"
    };

    println!("{}, check {}", roll_str, result);
}
