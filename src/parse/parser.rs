//! Parser takes in a token steam and outputs commands.

use std::iter::Peekable;

use super::cmd::{Cmd, CmdError, SimpleCmd};
use crate::prelude::*;

/// The parser reads in tokens and converts them into commands.
pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn get_next(&mut self) -> Result<Cmd, CmdError> {
        let mut cmd = SimpleCmd::default();
        loop {
            let Some(token) = self.tokens.next() else {
                return Ok(cmd.build());
            };

            match token {
                Token::Pipe => todo!(),
                Token::Amp => todo!(),
                Token::SemiColor => todo!(),
                Token::LeftArrow => todo!(),
                Token::RightArrow => todo!(),
                Token::OpenParen => todo!(),
                Token::CloseParen => todo!(),
                Token::Doller => todo!(),
                Token::BackTick(_) => todo!(),
                Token::Escape(_) => todo!(),
                Token::DoubleQuote(_) => todo!(),
                Token::SingleQuote(_) => todo!(),
                Token::Space => todo!(),
                Token::Tab => todo!(),
                Token::Newline => todo!(),
                Token::Glob => todo!(),
                Token::OpenBraket => todo!(),
                Token::Pound => todo!(),
                Token::Tilde => todo!(),
                Token::Equal => todo!(),
                Token::Percent => todo!(),
                Token::Ident(d) => cmd.push_ident(d),

                // Cant Start an expression
                Token::Huh => todo!(),
            }
        }
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<Cmd, CmdError>;

    fn next(&mut self) -> Option<Self::Item> {
        // when there are no tokens left return
        self.tokens.peek()?;

        Some(self.get_next())
    }
}
