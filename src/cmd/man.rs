use crate::parser::ParseResult;

pub fn parse(args: &Vec<String>) -> ParseResult {
    if args.len() == 0 {
        ParseResult::InvalidArgs("must specify command".to_owned())
    } else {
        match args[0].as_str() {
            _ => ParseResult::InvalidCmd(args[0].clone())
        }
    }
}