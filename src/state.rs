use std::collections::VecDeque;

use crate::parser::ParseResult;

#[derive(Default)]
pub struct State {
    history: VecDeque<String>,
}

impl State {
    pub fn get_cmd(&self, idx: usize) -> Option<String> {
        if idx < self.history.len() {
            Some(self.history[idx].clone())
        } else {
            None
        }
    }

    pub fn hist_len(&self) -> usize {
        self.history.len()
    }

    pub fn add_to_history(&mut self, cmd: &str) {
        if self.history.len() > 1000 {
            self.history.pop_front();
        }

        self.history.push_back(cmd.to_owned());

        // println!("history is now {:?}", self.history);
    }

    pub fn print_history(&self) -> ParseResult {
        let start = self.hist_len().saturating_sub(50);
        (start..(start + 50).min(self.hist_len())).for_each(|idx| {
            print!("{}  {}", idx + 1, self.history[idx]);
        });
        ParseResult::Cont(true)
    }
}
