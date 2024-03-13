use crate::prelude::*;

use std::iter::Peekable;

/// entry point to turing some chars into a token
pub(crate) fn next_token<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Option<Token> {
    match chars.next()? {
        '\"' => read_double_quotes(chars),
        '\'' => read_single_quotes(chars),

        '#' => todo!("read comment"),
        '$' => todo!("get var or subshell"),
        '`' => todo!("make subshell"),

        '|' => Some(Token::Pipe),
        '>' => Some(Token::RightArrow),
        '<' => Some(Token::LeftArrow),
        '!' => Some(Token::Bang),
        '(' => Some(Token::OpenParen),
        ')' => Some(Token::CloseParen),
        '*' => Some(Token::Glob),
        '{' => Some(Token::OpenBraket),
        '}' => Some(Token::CloseBraket),
        '~' => Some(Token::Tilde),
        '=' => Some(Token::Equal),
        '%' => Some(Token::Percent),
        ' ' => Some(Token::Space),
        '&' => Some(Token::Amp),
        '?' => Some(Token::Huh),
        ';' => Some(Token::SemiColor),
        '\t' => Some(Token::Tab),
        '\n' => Some(Token::Newline),
        c => read_ident(chars, c),
    }
}

// if self.done {
//     // users of this iterator will frequently exahust the entire iterator
//     // to make a command which they will return. At which point they
//     // then unknowingly see if there is another command in ready to be
//     // made. This is a gaurd against that case.
//     return None;
// }
//
// let next = self.line.peek()?;
// log!("reading token: {:?}", next);
//
// if let Some(token) = as_token(*next) {
//     let _ = self.line.next();
//     Some(token)
// } else {
//     log!("{:?} looks like the start of an ident", next);
//     Some(Token::Ident(self.read_ident()))
// }

/// Reads chars into a buffer until it encounters a special character.
fn read_ident<I: Iterator<Item = char>>(chars: &mut Peekable<I>, c: char) -> Option<Token> {
    fn is_special(c: &char) -> bool {
        matches!(
            c,
            '<' | '>' | '(' | ')' | '$' | '`' | '"' | '\'' | '*' | '\n' | ' ' | '&' | '|' | ';'
        )
    }

    let mut s = String::new();
    s.push(c);

    while let Some(c) = chars.peek() {
        if is_special(c) {
            break;
        }
        s.push(*c);
        let _ = chars.next();
    }
    Some(Token::Ident(s))
}

fn read_double_quotes<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Option<Token> {
    let mut v = vec![];

    loop {
        let t = chars.next();
        match t {
            Some('\"') => return Some(Token::DoubleQuote(v)),
            Some('\\') => v.push(read_escape(chars)),
            Some(_) => todo!(),
            None => panic!("unclosed double quotes"),
        }
    }
}

fn read_single_quotes<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Option<Token> {
    read_raw_until(chars, |c| c != '\'', true).map(Token::SingleQuote)
}

fn read_escape<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Token {
    match chars.next() {
        Some('n') => Token::Ident(String::from("\n")),
        Some('\\') => Token::Ident(String::from("\\")),
        // Token::Ident(format!("\\{}", c)),
        Some(c) => todo!("handle escaping: {:?}", c),
        // HACK: I dont know what to do here:
        // bash -c "echo \\"
        // returns "\" so I think this is fine
        None => Token::Ident(String::from("\\")),
    }
}

/// Reads data while parsing the minimal amount until a condition has been
/// reached.
///
/// if must=true then it must read a character that matches the pattern
/// if must=false then this always returns Some
fn read_raw_until<I, F>(chars: &mut Peekable<I>, cond: F, must: bool) -> Option<String>
where
    I: Iterator<Item = char>,
    F: Fn(char) -> bool,
{
    let mut word = String::new();

    while let Some(c) = chars.peek() {
        match c {
            '\\' => {
                chars.next();
                match chars.next() {
                    Some('\n') => todo!(),
                    Some('\'') => word.push('\''),
                    Some(c) => {
                        word.push('\\');
                        word.push(c)
                    }
                    None => {}
                }
            }
            c if cond(*c) => return Some(word),
            _ => word.push(chars.next().unwrap()),
        }
    }
    (!must).then_some(word)
}

/// A convience wrapped that just calls [`next_token`] as an iterator.
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    line: Peekable<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Lexer<I> {
        Lexer {
            // shell,
            line: input.peekable(),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        info!("poll");
        next_token(&mut self.line)
    }
}

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
    /// `"text"`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <dollar-sign>{captures}
    /// - <backquote>{captures}
    /// - <backslash>{captures}
    DoubleQuote(Vec<Token>),
    /// `'<ident>'`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <backslash><singlequote>
    /// - <backslash><backslash><singlequote>
    SingleQuote(String),
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
    /// `!`
    Bang,
    /// `~`
    Tilde,
    /// `=`
    Equal,
    /// `%`
    Percent,
    /// Anything Else
    // Ident(&'a str),
    Ident(String),
    /// Any sub shell. This is sepurated early in the pipeline so it is not
    /// parsed twice.
    Sub(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Pipe => f.write_str("|"),
            Token::Amp => f.write_str("&"),
            Token::SemiColor => f.write_str(";"),
            Token::LeftArrow => f.write_str("<"),
            Token::RightArrow => f.write_str(">"),
            Token::OpenParen => f.write_str("("),
            Token::CloseParen => f.write_str(")"),
            Token::Doller => f.write_str("$"),
            Token::BackTick => f.write_str("`"),
            Token::DoubleQuote(v) => {
                f.write_str("\"")?;
                for t in v {
                    write!(f, "{}", t)?;
                }
                f.write_str("\"")
            }
            Token::SingleQuote(s) => write!(f, "'{}'", s),
            Token::Space => f.write_str(" "),
            Token::Tab => f.write_str("\t"),
            Token::Newline => f.write_str("\n"),
            Token::Glob => f.write_str("*"),
            Token::Huh => f.write_str("?"),
            Token::OpenBraket => todo!(),
            Token::CloseBraket => todo!(),
            Token::Pound => f.write_str("#"),
            Token::Bang => f.write_str("!"),
            Token::Tilde => f.write_str("~"),
            Token::Equal => f.write_str("="),
            Token::Percent => f.write_str("%"),
            Token::Ident(s) => f.write_str(s),
            Token::Sub(s) => write!(f, "$({})", s),
        }
    }
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
