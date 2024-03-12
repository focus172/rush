use crate::prelude::*;

/// Repersents a Token from the input. The input must outlive this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    /// `|`
    Pipe,
    /// `&`
    Amp,
    /// `;`
    SemiColor,
    /// `<`
    LeftArrow,
    /// `>`
    RightArrow,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `$`
    Doller,
    /// '`'
    BackTick,
    /// `\c`
    Escape(char),
    /// `"text"`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <dollar-sign>{captures}
    /// - <backquote>{captures}
    /// - <backslash>{captures}
    DoubleQuote,
    /// `'<ident>'`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <backslash><singlequote>
    /// - <backslash><backslash><singlequote>
    SingleQuote,
    /// ` `
    Space,
    /// `\t`
    Tab,
    /// `\n`
    Newline,
    /// `*`
    Glob,
    /// `?`
    Huh,
    /// `{`
    OpenBraket,
    /// `}`
    CloseBraket,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `=`
    Equal,
    /// `%`
    Percent,
    /// Anything Else
    // Ident(&'a str),
    Ident(String),
}

use std::iter::Peekable;

/// An iterator wrapper that converts a char steam to token steam.
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

        // '\\' => {
        //     let _ = self.line.next(); // the `\` character
        // }
        loop {
            let Some(c) = self.line.peek() else {
                return buf;
            };
            let c = *c;
            if let Some(_) = as_token(c) {
                return buf;
            }
            if c == '\\' {
                let _ = self.line.next();
                if let Some(_) = self.line.next() {
                    let _ = todo!("escape sequences.");
                } else {
                    // HACK: i dont know what to do here:
                    // bash -c "echo \\"
                    // returns "\" so I think this is fine
                    buf.push('\\');
                    return buf;
                }
            } else {
                buf.push(c);
                let _ = self.line.next();
            }
        }
    }
}

fn as_token(c: char) -> Option<Token> {
    match c {
        '<' => Some(Token::LeftArrow),
        '>' => Some(Token::RightArrow),
        '(' => Some(Token::OpenParen),
        ')' => Some(Token::CloseParen),
        '$' => Some(Token::Doller),
        '`' => Some(Token::BackTick),
        '"' => Some(Token::DoubleQuote),
        '\'' => Some(Token::SingleQuote),
        '\t' => Some(Token::Tab),
        '*' => Some(Token::Glob),
        '{' => Some(Token::OpenBraket),
        '}' => Some(Token::CloseBraket),
        '#' => Some(Token::Pound),
        '~' => Some(Token::Tilde),
        '=' => Some(Token::Equal),
        '%' => Some(Token::Percent),
        '\n' => Some(Token::Newline),
        ' ' => Some(Token::Space),
        '&' => Some(Token::Amp),
        '|' => Some(Token::Pipe),
        '?' => Some(Token::Huh),
        ';' => Some(Token::SemiColor),
        _ => None,
    }
}

// macro_rules! token {
//     ($read:expr, $token:expr) => {{
//         let iter = &mut $read;
//         let _ = Iterator::next(iter);
//         Some($token)
//     }};
// }

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            // users of this iterator will frequently exahust the entire iterator
            // to make a command which they will return. At which point they
            // then unknowingly see if there is another command in ready to be
            // made. This is a gaurd against that case.
            return None;
        }

        let next = self.line.peek()?;
        log!("reading token: {:?}", next);

        if let Some(token) = as_token(*next) {
            let _ = self.line.next();
            Some(token)
        } else {
            log!("{:?} looks like the start of an ident", next);
            Some(Token::Ident(self.read_ident()))
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

#[cfg(test)]
mod test {
    use super::Lexer;
    use super::Token;

    #[test]
    fn lexer() {
        let input = String::from("exa -1 | grep cargo");
        let mut lexer = Lexer::new(input.chars());
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
