use std::sync::LazyLock;

use linefeed::{Completer, Completion, Prompter, Terminal};

pub struct DndshCompleter;

impl<T: Terminal> Completer<T> for DndshCompleter {
    fn complete(&self, word: &str, _prompter: &Prompter<T>, _start: usize, _end: usize) -> Option<Vec<Completion>> {
        static CMDS: LazyLock<Vec<&str>> = LazyLock::new(||vec!["roll", "check", "history", "version", "exit", "quit"]);
        let completions: Vec<Completion> = CMDS
            .iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|cmd| Completion::simple(cmd.to_string()))
            .collect();

        if completions.is_empty() {
            None
        } else {
            Some(completions)
        }
    }
}