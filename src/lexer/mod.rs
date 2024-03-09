//! The lexer parses text into tokens.

// use crate::helpers::Shell;
// use self::Expand::*;

mod token;
pub use token::Token;

use std::iter::Peekable;

/// An iterator wrapper that converts any char steam to token steam.
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    line: Peekable<I>,
    done: bool,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Lexer<I> {
        Lexer {
            // shell,
            line: input.peekable(),
            done: false,
        }
    }

    fn read_ident(&mut self) -> String {
        let mut buf = String::new();

        loop {
            let Some(c) = self.line.peek() else {
                return buf;
            };
            // move out of the reference
            let c = *c;

            match c {
                ' ' | '\n' => return buf,
                _ => {
                    let _ = self.line.next();
                    buf.push(c)
                }
            }
        }
    }
}

macro_rules! token {
    ($read:expr, $token:expr) => {{
        let iter = &mut $read;
        let _ = Iterator::next(iter);
        Some($token)
    }};
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            // users of this iterator will frequently equast the entire iterator
            // to make a command which they will return. At which point they
            // then unknowingly see if there is another command in ready to be
            // made. This is a gaurd against that case.
            return None;
        }

        let next = self.line.peek()?;
        log::trace!("reading token");

        match next {
            '<' => token!(self.line, Token::LeftArrow),
            '>' => token!(self.line, Token::RightArrow),
            '(' => token!(self.line, Token::OpenParen),
            ')' => token!(self.line, Token::CloseParen),
            '$' => token!(self.line, Token::Doller),
            '`' => token!(self.line, Token::BackTick),
            '"' => token!(self.line, Token::DoubleQuote),
            '\'' => token!(self.line, Token::SingleQuote),
            '\t' => token!(self.line, Token::Tab),
            '*' => token!(self.line, Token::Glob),
            '{' => token!(self.line, Token::OpenBraket),
            '}' => token!(self.line, Token::CloseBraket),
            '#' => token!(self.line, Token::Pound),
            '~' => token!(self.line, Token::Tilde),
            '=' => token!(self.line, Token::Equal),
            '%' => token!(self.line, Token::Percent),
            '\n' => token!(self.line, Token::Newline),
            ' ' => token!(self.line, Token::Space),
            '&' => token!(self.line, Token::Amp),
            '|' => token!(self.line, Token::Pipe),
            '?' => token!(self.line, Token::Huh),
            ';' => token!(self.line, Token::SemiColor),
            '\\' => {
                let _ = self.line.next(); // the `\` character
                if let Some(c) = self.line.next() {
                    Some(Token::Escape(c))
                } else {
                    // HACK: i dont know what to do here:
                    // bash -c "echo \\"
                    // returns "\" so I think this is fine
                    Some(Token::Ident(String::from("\\")))
                }
            }
            _ => Some(Token::Ident(self.read_ident())),
        }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    // fn skip_whitespace(&mut self) {
    //     let mut next = self.peek_char();
    //     while next.is_some() && next.unwrap().is_whitespace() {
    //         self.next_char();
    //         next = self.peek_char();
    //     }
    // }
    //
    // fn read_until(
    //     &mut self,
    //     consume: bool,
    //     keep_going: bool,
    //     split_on_space: bool,
    //     break_cond: impl Fn(char) -> bool,
    //     shell: &mut Shell,
    // ) -> Result<Vec<Expand>, String> {
    //     let mut expandables = Vec::new();
    //     let mut cur_word = String::new();
    //
    //     let mut next = self.peek_char();
    //     loop {
    //         match next {
    //             Some('\\') => {
    //                 self.next_char();
    //                 match self.next_char() {
    //                     Some('\n') => self.advance_line(shell)?,
    //                     Some(c) => cur_word.push(c),
    //                     None => (),
    //                 }
    //             }
    //             Some(c) if break_cond(*c) => {
    //                 // This just makes assignment easier
    //                 if *c == '=' {
    //                     cur_word.push(self.next_char().unwrap());
    //                     expandables.push(Literal(cur_word));
    //                     cur_word = String::new();
    //                 } else {
    //                     if consume {
    //                         self.next_char();
    //                     }
    //                     break;
    //                 }
    //             }
    //             Some(' ') if split_on_space => {
    //                 self.next_char();
    //                 if !cur_word.is_empty() {
    //                     expandables.push(Literal(cur_word));
    //                     cur_word = String::new();
    //                 }
    //             }
    //             Some('$') => {
    //                 if !cur_word.is_empty() {
    //                     expandables.push(Literal(cur_word));
    //                     cur_word = String::new();
    //                 }
    //                 self.next_char();
    //                 match self.peek_char() {
    //                     Some('{') => {
    //                         fn get_action(null: bool, c: Option<char>) -> Option<Action> {
    //                             match c {
    //                                 Some('-') => Some(Action::UseDefault(null)),
    //                                 Some('=') => Some(Action::AssignDefault(null)),
    //                                 Some('?') => Some(Action::IndicateError(null)),
    //                                 Some('+') => Some(Action::UseAlternate(null)),
    //                                 _ => None,
    //                             }
    //                         }
    //
    //                         self.next_char();
    //                         let param = self.read_raw_until(invalid_var, shell)?;
    //
    //                         let action = match self.next_char() {
    //                             Some(':') => get_action(true, self.next_char()),
    //                             Some('%') => {
    //                                 if let Some('%') = self.peek_char() {
    //                                     self.next_char();
    //                                     Some(Action::RmLargestSuffix)
    //                                 } else {
    //                                     Some(Action::RmSmallestSuffix)
    //                                 }
    //                             }
    //                             Some('#') => {
    //                                 if let Some('#') = self.peek_char() {
    //                                     self.next_char();
    //                                     Some(Action::RmLargestPrefix)
    //                                 } else {
    //                                     Some(Action::RmSmallestPrefix)
    //                                 }
    //                             }
    //                             Some(' ') => return Err(String::from("bad substitution")),
    //                             c => get_action(false, c),
    //                         };
    //
    //                         if let Some(a) = action {
    //                             let word = self.read_until(
    //                                 true,
    //                                 true,
    //                                 false,
    //                                 Box::new(|c| c == '}'),
    //                                 shell,
    //                             )?;
    //                             expandables.push(Brace(param, a, word));
    //                         } else {
    //                             expandables.push(Var(param));
    //                         }
    //                     }
    //                     Some('(') => {
    //                         self.next_char();
    //                         expandables.push(Sub(self.read_until(
    //                             true,
    //                             true,
    //                             true,
    //                             Box::new(|c| c == ')'),
    //                             shell,
    //                         )?));
    //                     }
    //                     Some('$') => {
    //                         // '$$' command doesn't play nicely with the reading here,
    //                         // but it's so simple I can just check for it here.
    //                         self.next_char();
    //                         expandables.push(Var(String::from("$")));
    //                     }
    //                     _ => {
    //                         expandables.push(Var(self.read_raw_until(invalid_var, shell)?));
    //                     }
    //                 }
    //             }
    //             Some('`') => {
    //                 // How often are backticks actually used for subshells?
    //                 // I really don't want to have to implement nested backtick subshells...
    //                 self.next_char();
    //                 expandables.push(Sub(self.read_until(
    //                     true,
    //                     true,
    //                     true,
    //                     Box::new(|c| c == '`'),
    //                     shell,
    //                 )?));
    //             }
    //             Some('~') => {
    //                 if !cur_word.is_empty() {
    //                     expandables.push(Literal(cur_word));
    //                     cur_word = String::new();
    //                 }
    //                 self.next_char();
    //
    //                 let tilde =
    //                     self.read_until(false, false, false, Box::new(invalid_var), shell)?;
    //                 expandables.push(Tilde(tilde));
    //             }
    //             Some('"') => {
    //                 if !cur_word.is_empty() {
    //                     expandables.push(Literal(cur_word));
    //                     cur_word = String::new();
    //                 }
    //                 self.next_char();
    //
    //                 let mut result =
    //                     self.read_until(true, true, false, Box::new(|c| c == '"'), shell)?;
    //                 if result.is_empty() {
    //                     expandables.push(Literal(String::new()));
    //                 } else {
    //                     expandables.append(&mut result);
    //                 }
    //             }
    //             Some('\'') => {
    //                 self.next_char();
    //                 let mut phrase = String::new();
    //                 loop {
    //                     match self.next_char() {
    //                         Some('\'') => break,
    //                         Some(c) => phrase.push(c),
    //                         None => self.advance_line(shell)?,
    //                     }
    //                 }
    //                 expandables.push(Literal(phrase));
    //             }
    //             Some(_) => cur_word.push(self.next_char().unwrap()),
    //             None => {
    //                 if keep_going {
    //                     self.advance_line(shell)?;
    //                 } else {
    //                     break;
    //                 }
    //             }
    //         }
    //         next = self.peek_char();
    //     }
    //     if !cur_word.is_empty() {
    //         expandables.push(Literal(cur_word));
    //     }
    //     Ok(expandables)
    // }
    //
    // // You can accomplish this same thing with just the function above and some matching/unwrapping,
    // // but I think this is cleaner
    // fn read_raw_until<F>(&mut self, break_cond: F, shell: &mut Shell) -> Result<String, String>
    // where
    //     F: Fn(char) -> bool,
    // {
    //     let mut word = String::new();
    //     while let Some(c) = self.peek_char() {
    //         match c {
    //             '\\' => {
    //                 self.next_char();
    //                 match self.next_char() {
    //                     Some('\n') => self.advance_line(shell)?,
    //                     Some(c) => word.push(c),
    //                     None => (),
    //                 }
    //             }
    //             c if break_cond(*c) => break,
    //             _ => word.push(self.next_char().unwrap()),
    //         }
    //     }
    //     Ok(word)
    // }
    //
    // // Of course, I still haven't added everything I'll need to yet
    // pub fn next_token(&mut self, shell: &mut Shell) -> Option<Token> {
    //     self.skip_whitespace();
    //     match self.peek_char() {
    //         Some('|') => {
    //             self.next_char();
    //             if let Some('|') = self.peek_char() {
    //                 self.next_char();
    //                 Some(Token::Op(Op::Or))
    //             } else {
    //                 Some(Token::Op(Op::Pipe))
    //             }
    //         }
    //         Some('&') => {
    //             self.next_char();
    //             if let Some('&') = self.peek_char() {
    //                 self.next_char();
    //                 Some(Token::Op(Op::And))
    //             } else {
    //                 Some(Token::Op(Op::Ampersand))
    //             }
    //         }
    //         Some('>') => {
    //             self.next_char();
    //             Some(Token::Op(Op::More))
    //         }
    //         Some('<') => {
    //             self.next_char();
    //             Some(Token::Op(Op::Less))
    //         }
    //         Some('!') => {
    //             self.next_char();
    //             Some(Token::Op(Op::Bang))
    //         }
    //         Some('(') => {
    //             self.next_char();
    //             Some(Token::Punct(Punct::LParen))
    //         }
    //         Some(')') => {
    //             self.next_char();
    //             Some(Token::Punct(Punct::RParen))
    //         }
    //         Some(_) => {
    //             match self.read_until(false, false, false, Box::new(is_token_split), shell) {
    //                 Ok(w) => {
    //                     println!("The words I got: {:?}", w);
    //                     match &w[..] {
    //                         [Literal(s), ..]
    //                             if s.ends_with('=')
    //                                 && s.chars().filter(|c| c.is_numeric()).count()
    //                                     != s.len() - 1 =>
    //                         {
    //                             let mut iter = w.into_iter();
    //                             let mut name = iter.next().unwrap().get_name();
    //                             name.pop();
    //                             Some(Token::Assign(name, iter.collect()))
    //                         }
    //                         [Literal(s)] => {
    //                             if let Ok(num) = s.parse::<u32>() {
    //                                 Some(Token::Integer(num))
    //                             } else {
    //                                 Some(Token::Word(w))
    //                             }
    //                         }
    //                         _ => Some(Token::Word(w)),
    //                     }
    //                 }
    //                 Err(e) => {
    //                     eprintln!("rush: {}", e);
    //                     None
    //                 }
    //             }
    //         }
    //         None => None,
    //     }
    // }
}

// #[derive(Debug, PartialEq)]
// pub enum Expand {
//     Literal(String),
//     Var(String),
//     Tilde(Vec<Expand>),
//     Brace(String, Action, Vec<Expand>),
//     Sub(Vec<Expand>),
// }

/// What the brace does expansion does:
/// https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_02
/// If true test for unset or null, if false, only unset
/// For prefix/suffix, true for largest false for smallest
#[derive(Debug, PartialEq)]
pub enum ExpandAction {
    // UseDefault(bool),
    // AssignDefault(bool),
    // IndicateError(bool),
    // UseAlternate(bool),
    // RmSmallestSuffix,
    // RmLargestSuffix,
    // RmSmallestPrefix,
    // RmLargestPrefix,
    // StringLength,
}

// impl Expand {
//     pub fn get_name(self) -> String {
//         match self {
//             Literal(s) | Var(s) | Brace(s, _, _) => s,
//             Tilde(_) | Sub(_) => panic!("you shouldn't be doing this"),
//         }
//     }
// }

// // Operators
// #[derive(Debug, PartialEq)]
// pub enum Op {
//     Pipe,
//     Ampersand,
//     Bang,
//     Or,
//     And,
//     Less,
//     More,
// }

// // Punctuation
// #[derive(Debug, PartialEq)]
// pub enum Punct {
//     LParen,
//     RParen,
//     Semicolon,
// }

#[cfg(test)]
mod test {
    use super::Lexer;
    use super::Token;
    use crate::util::char::OwnedChars;

    #[test]
    fn lex() {
        let input = String::from("exa -1 | grep cargo");
        let mut lexer = Lexer::new(OwnedChars::new(input));
        let expected = [
            Token::Ident(String::from("exa")),
            Token::Space,
            Token::Ident(String::from("-1")),
            Token::Space,
            Token::Pipe,
            Token::Space,
            Token::Ident(String::from("grep")),
            Token::Space,
            Token::Ident(String::from("cargo")),
        ];
        for token in expected {
            assert_eq!(Some(token), lexer.next())
        }
    }
}
