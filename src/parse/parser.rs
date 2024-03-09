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

            // Todo: things build args and then spaces deliminate them.
            // This way args can contain variables and other as many tokens
            // not one. This reduces the load on the tokenizer.

            match token {
                Token::Newline => return Ok(cmd.build()),

                Token::Pipe => {
                    let c = cmd.build();
                    return Ok(Cmd::Pipeline(Box::new(c), Box::new(self.get_next()?)));
                }
                Token::Amp => todo!(),
                Token::SemiColor => todo!(),
                Token::LeftArrow => todo!(),
                Token::RightArrow => todo!(),
                Token::OpenParen => todo!(),
                Token::CloseParen => todo!(),
                Token::Doller => todo!(),
                Token::BackTick => todo!(),
                Token::Escape(_) => todo!(),
                Token::DoubleQuote => todo!(),
                Token::SingleQuote => todo!(),
                Token::Tab => todo!(),
                Token::Glob => todo!(),
                Token::OpenBraket => todo!(),
                Token::CloseBraket => todo!(),
                Token::Pound => todo!(),
                Token::Tilde => todo!(),
                Token::Equal => todo!(),
                Token::Percent => todo!(),
                Token::Ident(d) => cmd.push_ident(d),

                Token::Space => {}

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

        log::trace!("getting next command.");

        Some(self.get_next())
    }
}
