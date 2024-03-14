use std::iter::Peekable;

use crate::prelude::*;

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
        while let Some(token) = self.tokens.next() {
            if matches!(token, Token::Newline) {
                break;
            }
        }
        String::new()
    }

    // Reads some text. This is by far the most powerful of the reads. The
    // best way to understand this is that if it could be on either side of
    // an equal sign and make sence then here it is.
    // fn read_expression(&mut self) -> Vec<Expand> {
    //     todo!()
    // }

    // Polls the next value from this token walker. A return value of `None`
    // from this funtion means that there is not enough info to produce a
    // token yet. A return value of `Some(None)` means there are no more
    // tokens that can be produced.
    //
    // This will only happen if this has been called.
    // pub fn poll_next(&mut self, next: Option<Token>) -> Option<Option<TreeItem>> {
    //     todo!()
    // }
}

impl TryFrom<Vec<Expand>> for TreeItem {
    type Error = ();

    fn try_from(value: Vec<Expand>) -> std::result::Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(TreeItem::Word(value))
        }
    }
}

impl<I> Iterator for Walker<I>
where
    I: Iterator<Item = Token>,
{
    type Item = TreeItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut expr = vec![];
        while let Some(t) = self.tokens.peek() {
            match t {
                Token::Newline => {
                    has!(TreeItem::try_from(expr).ok());
                    let _ = self.tokens.next();
                    return Some(TreeItem::StatmentEnd);
                }

                Token::Pipe => {
                    let _ = self.tokens.next();
                    if let Some(Token::Pipe) = self.tokens.peek() {
                        let _ = self.tokens.next();
                        return Some(TreeItem::Or);
                    }
                    return Some(TreeItem::Pipe);
                }
                Token::Amp => {
                    let _ = self.tokens.next();
                    match self.tokens.peek() {
                        Some(Token::Amp) => {
                            let _ = self.tokens.next();
                            return Some(TreeItem::And);
                        }
                        Some(_) => todo!(),
                        None => return Some(TreeItem::Background),
                    }
                }
                Token::SemiColor => todo!(),
                Token::LeftArrow => todo!(),
                Token::RightArrow => {
                    // dont consume the token if we already have something
                    // buffered
                    has!(TreeItem::try_from(expr).ok());

                    // now it can go
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
                            expr.push(Expand::Var(s))
                        }
                        Some(Token::Space) => {
                            expr.push(Expand::Literal(String::from("$")));
                        }
                        // this is handled in tokenization
                        Some(Token::OpenParen) => unreachable!(),
                        Some(_) => todo!(),
                        None => todo!(),
                    }
                }
                Token::BackTick => todo!(),
                Token::DoubleQuote(_) => {
                    let Some(Token::DoubleQuote(v)) = self.tokens.next() else {
                        unreachable!()
                    };
                    dbg!(&v);
                    let mut v = v.into_iter();
                    let mut e = vec![];
                    while let Some(t) = v.next() {
                        match t {
                            Token::Doller => {
                                if let Some(Token::Ident(s)) = v.next() {
                                    e.push(Expand::Var(s));
                                } else {
                                    e.push(Expand::Literal(String::from("")));
                                }
                            }
                            Token::Ident(s) => e.push(Expand::Literal(s)),
                            Token::Sub(_) => todo!("evaluate this shit"),
                            _ => unreachable!("bad token in double quotes"),
                        }
                    }
                    return Some(TreeItem::Word(e));
                }
                Token::SingleQuote(_) => {
                    let Some(Token::SingleQuote(s)) = self.tokens.next() else {
                        unreachable!()
                    };
                    expr.push(Expand::Literal(s))
                }
                Token::Tab => todo!(),
                Token::Glob => todo!(),
                Token::OpenBraket => todo!(),
                Token::CloseBraket => todo!(),
                Token::Pound => {
                    let c = self.read_comment();
                    info!("got comment: {:?}", c);
                    return Some(TreeItem::Comment);
                }
                Token::Tilde => {
                    warn!("doing bad expansion of any tilde to home");
                    expr.push(Expand::Home);
                }
                Token::Equal => {
                    // let a = TreeItem::Assign(expr, todo!());
                    todo!()
                }
                Token::Percent => todo!(),
                Token::Ident(_) => {
                    let Some(Token::Ident(s)) = self.tokens.next() else {
                        unreachable!()
                    };
                    expr.push(Expand::Literal(s));
                }

                Token::Space => {
                    let _ = self.tokens.next();
                    if !expr.is_empty() {
                        return Some(TreeItem::Word(expr));
                    }
                }
                Token::Sub(_) => {
                    let Some(Token::Sub(s)) = self.tokens.next() else {
                        unreachable!()
                    };
                    expr.push(Expand::Sub(s))
                }
                Token::Bang => todo!(),

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
    // Assign(Vec<Expand>, Vec<Expand>),
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
    /// `# *[`Token`]`
    Comment, // (String),
    /// a ';' of '\n'
    StatmentEnd,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expand {
    Literal(String),
    Var(String),
    /// `~`
    Home,
    // Brace(String, ExpandAction, Vec<Expand>),
    Sub(String),
}

impl Expand {
    pub fn expand(self, state: &ShellState) -> String {
        match self {
            Expand::Literal(s) => s,
            Expand::Var(k) => {
                info!("explanding key: {}", k);
                // let (var, rest) = state.get_env(&k);
                // format!("{}{}", var, rest)

                state.get_env_exact(&k).unwrap_or(String::new())
            }
            Expand::Home => state.home().to_owned(),
            // Expand::Brace(_, _, _) => todo!(),
            Expand::Sub(s) => {
                let s = Shell::sourced(crate::lexer::Lexer::new(
                    crate::util::OwnedCharBuffer::new(s),
                ));
                s.run(false).unwrap();
                todo!("get shell output")
            }
        }
    }

    //     fn expand_word(&mut self, expansions: Vec<Expand>) -> String {
    //         let mut phrase = String::new();
    //         for word in expansions {
    //             match word {
    //                 Literal(s) => phrase.push_str(&s),
    //                 Tilde(word) => {
    //                     let s = self.expand_word(word);
    //                     if s.is_empty() || s.starts_with('/') {
    //                         phrase.push_str(&env::var("HOME").unwrap());
    //                         phrase.push_str(&s);
    //                     } else {
    //                         let mut strings = s.splitn(1, '/');
    //                         let name = strings.next().unwrap();
    //                         if let Some(user) = User::from_name(name).unwrap() {
    //                             phrase.push_str(user.dir.as_os_str().to_str().unwrap());
    //                             if let Some(path) = strings.next() {
    //                                 phrase.push_str(path);
    //                             }
    //                         } else {
    //                             phrase.push('~');
    //                             phrase.push_str(name);
    //                         }
    //                     }
    //                 }
    //                 Var(s) => {
    //                     phrase.push_str(&self.shell.borrow().get_var(&s).unwrap_or_default());
    //                 }
    //                 Brace(key, action, word) => {
    //                     let val = self.shell.borrow().get_var(&key);
    //                     match action {
    //                         Action::UseDefault(null) => {
    //                             if let Some(s) = val {
    //                                 if s == "" && null {
    //                                     phrase.push_str(&self.expand_word(word))
    //                                 } else {
    //                                     phrase.push_str(&s)
    //                                 }
    //                             } else {
    //                                 phrase.push_str(&self.expand_word(word))
    //                             }
    //                         }
    //                         Action::AssignDefault(null) => {
    //                             if let Some(s) = val {
    //                                 if s == "" && null {
    //                                     let expanded = self.expand_word(word);
    //                                     phrase.push_str(&expanded);
    //                                     self.shell.set_var(key, expanded);
    //                                 } else {
    //                                     phrase.push_str(&s)
    //                                 }
    //                             } else {
    //                                 let expanded = self.expand_word(word);
    //                                 phrase.push_str(&expanded);
    //                                 self.shell.set_var(key, expanded);
    //                             }
    //                         }
    //                         Action::IndicateError(null) => {
    //                             if let Some(s) = val {
    //                                 if s == "" && null {
    //                                     let message = self.expand_word(word);
    //                                     if message.is_empty() {
    //                                         eprintln!("rush: {}: parameter null", key);
    //                                     } else {
    //                                         eprintln!("rush: {}: {}", key, message);
    //                                     }
    //                                     if !self.shell.is_interactive() {
    //                                         exit(1);
    //                                     }
    //                                 } else {
    //                                     phrase.push_str(&s)
    //                                 }
    //                             } else {
    //                                 let message = self.expand_word(word);
    //                                 if message.is_empty() {
    //                                     eprintln!("rush: {}: parameter not set", key);
    //                                 } else {
    //                                     eprintln!("rush: {}: {}", key, message);
    //                                 }
    //                                 if !self.shell.is_interactive() {
    //                                     exit(1);
    //                                 }
    //                             }
    //                         }
    //                         Action::UseAlternate(null) => {
    //                             if let Some(s) = val {
    //                                 if s != "" || !null {
    //                                     phrase.push_str(&self.expand_word(word))
    //                                 }
    //                             }
    //                         }
    //                         Action::RmSmallestSuffix => todo!(),
    //                         Action::RmLargestSuffix => todo!(),
    //                         Action::RmSmallestPrefix => todo!(),
    //                         Action::RmLargestPrefix => todo!(),
    //                         Action::StringLength => todo!(),
    //                     }
    //                 }
    //                 Sub(e) => {
    //                     todo!("{:?}", e)
    //                     // // FIXME: `$(ls something)`, commands with params don't work atm
    //                     // // for some reason
    //                     //
    //                     // let mut parser = Parser::new(vec!(Word(e)).into_iter(), Rc::clone(&self.shell));
    //                     //
    //                     // // This setup here allows me to do a surprisingly easy subshell.
    //                     // // Though subshells typically seem to inherit everything I'm keeping in my
    //                     // // `shell` variable at the moment?
    //                     // if let Ok(command) = parser.get() {
    //                     //     #[cfg(debug_assertions)] // Only include when not built with `--release` flag
    //                     //     println!("\u{001b}[33m{:#?}\u{001b}[0m", command);
    //                     //
    //                     //     let mut output = Runner::new(Rc::clone(&parser.shell)).execute(command, true).unwrap();
    //                     //     output = output.replace(char::is_whitespace, " ");
    //                     //     phrase.push_str(output.trim());
    //                     // }
    //                 }
    //             }
    //         }
    //         phrase
    //     }
}

/// What the brace does expansion does:
/// https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_02
/// If true test for unset or null, if false, only unset
/// For prefix/suffix, true for largest false for smallest
#[derive(Debug, PartialEq, Eq)]
pub enum ExpandAction {
    UseDefault(bool),
    // AssignDefault(bool),
    // IndicateError(bool),
    // UseAlternate(bool),
    // RmSmallestSuffix,
    // RmLargestSuffix,
    // RmSmallestPrefix,
    // RmLargestPrefix,
    // StringLength,
    /// ${var}
    None,
}
