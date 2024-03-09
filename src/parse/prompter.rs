//! Prompter reads user input. Then creates a [`Parser`] to turn it into
//! commands.

use std::io::Write;

use crate::util::char::OwnedChars;
use crate::{lexer::Lexer, prelude::*};

use super::cmd::{Cmd, CmdError};
use super::parser::Parser;

#[derive(Default)]
pub struct Prompter {
    commads: Option<Box<dyn Iterator<Item = Result<Cmd, CmdError>>>>,
}

impl Iterator for Prompter {
    type Item = Result<Cmd, CmdError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(cmd) = self.commads.as_mut().and_then(|i| i.next()) {
                return Some(cmd);
            }

            print!("$> ");
            io::stdout().flush().unwrap();
            let s = std::io::stdin();
            let mut line = String::new();
            s.read_line(&mut line).unwrap();
            // let line = std::io::stdin().lines().next()?.unwrap();

            log::info!("got line: {}", line.trim());

            let p = Parser::new(Lexer::new(OwnedChars::new(line)));
            _ = self.commads.insert(Box::new(p));
        }
    }
}
