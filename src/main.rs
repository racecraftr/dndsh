use std::sync::Arc;

mod cmd;
mod parser;
mod state;
mod completer;
use completer::DndshCompleter;
use linefeed::{self, Interface, ReadResult};
use parser::{ParseResult, parse};
use state::State;

fn main() {
    _ = repl();
}

fn repl() -> std::io::Result<()> {

    let mut shell_state = State::default();

    let reader = Interface::new("my-repl")?;
    reader.set_prompt("dndsh> ")?;
    reader.set_completer(Arc::new(DndshCompleter));
    'run: while let ReadResult::Input(input) = reader.read_line()? {
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
    }

    Ok(())
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
