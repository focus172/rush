use std::iter::Peekable;

use crate::{log, shell::ShellState, Token};

pub(crate) struct Walker<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Walker<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn read_comment(&mut self) -> String {
        todo!()
    }

    fn read_subshell(&mut self) -> Vec<Token> {
        let mut seen = 0;
        let mut goal = 1;
        let mut tokn = vec![];

        while let Some(token) = self.tokens.next() {
            match token {
                Token::OpenParen => goal += 1,
                Token::CloseParen => {
                    seen += 1;
                    if seen >= goal {
                        return tokn;
                    }
                }
                t => tokn.push(t),
            }
        }
        panic!("subshell missing closing braket");
    }
}

impl<I> Iterator for Walker<I>
where
    I: Iterator<Item = Token>,
{
    type Item = TreeItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut a = vec![];
        while let Some(t) = self.tokens.peek() {
            match t {
                Token::Newline => {
                    if !a.is_empty() {
                        return Some(TreeItem::Word(a));
                    }
                    let _ = self.tokens.next();
                    return Some(TreeItem::StatmentEnd);
                }

                Token::Pipe => {
                    let _ = self.tokens.next();
                    if let Some(Token::Pipe) = self.tokens.peek() {
                        let _ = self.tokens.next();
                        return Some(TreeItem::And);
                    }
                    return Some(TreeItem::Pipe);
                }
                Token::Amp => todo!(),
                Token::SemiColor => todo!(),
                Token::LeftArrow => todo!(),
                Token::RightArrow => {
                    if !a.is_empty() {
                        a.push(Expand::Literal(String::from(">")));
                        return Some(TreeItem::Word(a));
                    }
                    // redirect
                    let _ = self.tokens.next();
                    match self.tokens.peek() {
                        Some(Token::RightArrow) => {
                            let _ = self.tokens.next();
                            return Some(TreeItem::Append);
                        }
                        Some(Token::Space) | Some(Token::Ident(_)) => {
                            return Some(TreeItem::Redirect);
                        }
                        Some(_) => todo!("bad token"),
                        None => todo!("expected string. found eof"),
                    }
                }
                Token::OpenParen => todo!(),
                Token::CloseParen => todo!(),
                Token::Doller => {
                    let _ = self.tokens.next();
                    match self.tokens.peek() {
                        Some(Token::Ident(_)) => {
                            let Some(Token::Ident(s)) = self.tokens.next() else {
                                unreachable!()
                            };
                            a.push(Expand::Var(s))
                        }
                        Some(Token::Space) => {
                            a.push(Expand::Literal(String::from("$")));
                        }
                        Some(Token::OpenParen) => {
                            let _ = self.tokens.next();
                            return Some(TreeItem::Subshell(self.read_subshell()));
                        }
                        Some(_) => todo!(),
                        None => todo!(),
                    }
                }
                Token::BackTick => todo!(),
                Token::Escape(_) => todo!(),
                Token::DoubleQuote => todo!(),
                Token::SingleQuote => todo!(),
                Token::Tab => todo!(),
                Token::Glob => todo!(),
                Token::OpenBraket => todo!(),
                Token::CloseBraket => todo!(),
                Token::Pound => return Some(TreeItem::Comment(self.read_comment())),
                Token::Tilde => todo!(),
                Token::Equal => todo!(),
                Token::Percent => todo!(),
                Token::Ident(_) => {
                    let Some(Token::Ident(s)) = self.tokens.next() else {
                        unreachable!()
                    };
                    a.push(Expand::Literal(s));
                }

                Token::Space => {
                    let _ = self.tokens.next();
                    if !a.is_empty() {
                        return Some(TreeItem::Word(a));
                    }
                }

                // Cant Start an expression
                Token::Huh => todo!(),
            }
        }

        // TODO: this can drop data when there is no newline at the end of the file
        None
    }
}

#[derive(Debug)]
pub(crate) enum TreeItem {
    Word(Vec<Expand>),
    /// Matching [`Equals`]. Can be used when assigning a variable or
    /// making an env for a command.
    /// can also sometimes be in arg position where it can just be stringifyed
    Assign(Vec<Expand>, Vec<Expand>),
    Append,
    Redirect,
    /// `&`
    Background,
    /// `||`
    Or,
    /// `&&`
    And,
    /// `|`
    Pipe,
    /// `$( *[`ASTreeItem`] )`
    Subshell(Vec<Token>),
    /// `# *[`Token`]`
    Comment(String),
    /// a ';' of '\n'
    StatmentEnd,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expand {
    Literal(String),
    Var(String),
    /// `~`
    Home,
    // Brace(String, Action, Vec<Expand>),
    // Sub(Vec<Expand>),
}

impl Expand {
    pub fn expand(self, state: &ShellState) -> String {
        match self {
            Expand::Literal(s) => s,
            Expand::Var(k) => {
                log!("explanding key: {}", k);
                let (var, rest) = state.get_env(&k);
                format!("{}{}", var, rest)
            }
            Expand::Home => state.home().to_owned(),
        }
    }
}

/// What the brace does expansion does:
/// https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_02
/// If true test for unset or null, if false, only unset
/// For prefix/suffix, true for largest false for smallest
#[derive(Debug, PartialEq, Eq)]
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
