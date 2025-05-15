use std::io::{self, Write};

mod parser;
mod state;
mod cmd;
use parser::{ParseResult, parse};
use state::State;

fn main() {
    let mut input = String::new();
    let mut shell_state = State::default();
    'run: loop {
        print!("dndsh> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parse_result = parse(&input, &mut shell_state);
        match parse_result {
            ParseResult::Exit => {
                break 'run;
            }
            ParseResult::Cont(b) => {
                if b {
                    shell_state.add_to_history(&input)
                }
            }
            ParseResult::InvalidCmd(s) => println!("Invalid command: {}", s),
            ParseResult::InvalidArgs(s) => println!("Invalid arguments: {}", s),
        }
        input.clear();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_substr() {
        let s = "h";
        let t = &s[1..];
        println!("{}", t);
    }

    #[test]
    fn test_saturate() {
        assert_eq!(0_usize.saturating_sub(1), 0);
    }
}
