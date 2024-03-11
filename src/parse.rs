use crate::shell::ShellState;
use crate::util::OwnedCharBuffer;
use crate::util::StaticMap;
use crate::walker::TreeItem;
use crate::{lexer::Lexer, walker::Walker};

use std::iter::Peekable;
use std::process::Stdio;

use crate::prelude::*;

/// The parser reads in tokens and converts them into commands.
/// Parser takes in a token stream and outputs commands.
pub(crate) struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<Walker<I>>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens: Walker::new(tokens).peekable(),
        }
    }

    pub fn next(&mut self, state: &ShellState) -> Option<Result<Cmd, CmdError>> {
        // when there are no tokens left return
        self.tokens.peek()?;

        log!("getting next command.");

        Some(self.get_next(state))
    }

    fn get_next(&mut self, state: &ShellState) -> Result<Cmd, CmdError> {
        let mut cmd = SimpleCmd::default();
        loop {
            let Some(token) = self.tokens.next() else {
                return Ok(cmd.build());
            };
            log!("got token: {:?}", token);

            // Todo: things build args and then spaces deliminate them.
            // This way args can contain variables and other as many tokens
            // not one. This reduces the load on the tokenizer.

            match token {
                TreeItem::Word(v) => {
                    let a =
                        v.into_iter()
                            .map(|e| e.expand(state))
                            .fold(String::new(), |mut s, e| {
                                s.push_str(&e);
                                s
                            });
                    cmd.push_ident(a);
                }
                TreeItem::Or => {
                    let c = cmd.build();
                    return Ok(Cmd::Or(Box::new(c), Box::new(self.get_next(state)?)));
                }
                TreeItem::And => {
                    let c = cmd.build();
                    return Ok(Cmd::And(Box::new(c), Box::new(self.get_next(state)?)));
                }
                TreeItem::Pipe => {
                    let c = cmd.build();
                    return Ok(Cmd::Pipeline(Box::new(c), Box::new(self.get_next(state)?)));
                }
                TreeItem::Subshell(tokens) => {
                    let s = Shell::sourced(tokens.into_iter());
                    s.run(false).change_context(CmdError::SubShell)?;
                }
                TreeItem::StatmentEnd => return Ok(cmd.build()),
                TreeItem::Assign(_, _) => todo!(),
                TreeItem::Append => todo!(),
                TreeItem::Redirect => todo!(),
                TreeItem::Background => todo!(),
                TreeItem::Comment(_) => todo!(),
            }
        }
    }
}

/// Prompter reads user input. Then creates a [`Parser`] to turn it into
/// commands.
#[derive(Default)]
pub(crate) struct Prompter {
    commads: Option<Parser<Lexer<OwnedCharBuffer>>>,
}

impl Prompter {
    pub fn next(&mut self, state: &ShellState) -> Option<Result<Cmd, CmdError>> {
        use std::io::Write;

        loop {
            if let Some(cmd) = self.commads.as_mut().and_then(|i| i.next(state)) {
                return Some(cmd);
            }

            print!("$> ");
            io::stdout().flush().unwrap();
            let s = std::io::stdin();
            let mut line = String::new();
            s.read_line(&mut line).unwrap();
            // let line = std::io::stdin().lines().next()?.unwrap();

            log!("got line: {}", line.trim());

            let p = Parser::new(Lexer::new(OwnedCharBuffer::new(line)));
            _ = self.commads.insert(p);
        }
    }
}

#[derive(Debug)]
pub enum CmdError {
    BadToken(Token),
    MissingName,
    SubShell,
}
impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdError::BadToken(t) => write!(f, "invalid token recieved: {:?}", t),
            CmdError::MissingName => f.write_str("a name is needed to call it"),
            CmdError::SubShell => f.write_str("Error in subshell. This could recurse."),
        }
    }
}
impl Context for CmdError {}

/// If you want to create one
#[derive(Debug, PartialEq)]
pub enum Cmd {
    Simple(SimpleCmd),
    Pipeline(Box<Cmd>, Box<Cmd>),
    And(Box<Cmd>, Box<Cmd>),
    Or(Box<Cmd>, Box<Cmd>),
    Not(Box<Cmd>),
    Empty,
}

impl Cmd {
    // Internal only helper function to get command we really want to edit.
    // fn get_last_mut(&mut self) -> &mut SimpleCmd {
    //     match self {
    //         Cmd::Simple(s) => s,
    //         Cmd::Pipeline(_, c) => c.get_last_mut(),
    //         Cmd::And(_, c) => c.get_last_mut(),
    //         Cmd::Or(_, c) => c.get_last_mut(),
    //         Cmd::Not(c) => c.get_last_mut(),
    //     }
    // }
    //
    // fn get_last(&self) -> &SimpleCmd {
    //     match self {
    //         Cmd::Simple(s) => s,
    //         Cmd::And(_, c) => c.get_last(),
    //         Cmd::Pipeline(_, _) => todo!(),
    //         Cmd::Or(_, _) => todo!(),
    //         Cmd::Not(_) => todo!(),
    //     }
    // }

    //         let cmd = self.get_mut_builder();
    //         match token {
    //             Token::Pipe => todo!(),
    //             Token::Amp => todo!(),
    //             Token::SemiColor => {
    //                 // if we are at the end of the statement we can ignore a
    //                 // redundant semicolor
    //                 let Some(cmd) = cmd else { return Ok(()) };
    //                 *self = CmdBuilder::Statment(cmd.build_take()?);
    //                 Ok(())
    //             }
    //             Token::LeftArrow => todo!(),
    //             Token::RightArrow => todo!(),
    //             Token::OpenParen => todo!(),
    //             Token::CloseParen => todo!(),
    //             Token::Doller => todo!(),
    //             Token::BackTick(_) => todo!(),
    //             Token::Escape(_) => todo!(),
    //             Token::DoubleQuote(_) => todo!(),
    //             Token::SingleQuote(_) => todo!(),
    //             Token::Space => todo!(),
    //             Token::Tab => todo!(),
    //             Token::Newline => todo!(),
    //             Token::Glob => todo!(),
    //             Token::Huh => todo!(),
    //             Token::OpenBraket => todo!(),
    //             Token::Pound => todo!(),
    //             Token::Tilde => todo!(),
    //             Token::Equal => todo!(),
    //             Token::Percent => todo!(),
    //             Token::Ident(s) => match cmd {
    //                 Some(c) => {
    //                     c.push_ident(s);
    //                     Ok(())
    //                 }
    //                 None => todo!(),
    //             },
    //         }
    //     }

    // Pushes a token to this command. Returns [`Ok(true)`] if this
    // repersents a valid command.
    // pub fn push(&mut self, token: Token) -> Result<(), CmdError> {
    //     let cmd = self.get_last_mut();
    //     match cmd {
    //         Cmd::Simple(s) => match token {
    //             Token::Ident(value) => Ok(s.args.push(value)),
    //             _ => todo!(),
    //         },
    //         Cmd::Incomplete(icmd) => match token {
    //             Token::Ident(name) => {
    //                 // TODO: check if this is an env var assignment
    //                 // Ex:
    //                 // ```bash
    //                 // RUST_BACKTRACE=1 cargo run
    //                 // ```
    //                 // ----------------
    //                 //        |
    //                 //       this
    //                 Ok(*cmd = Cmd::Simple(icmd.build_take(name)))
    //             }
    //             t => Err(Report::new(CmdError::BadToken(t))),
    //         },
    //
    //         _ => unreachable!(),
    //     }
    // }
}

/// The most basic command - it, its arguments, and its redirections.
#[derive(Debug, Default, PartialEq)]
pub struct SimpleCmd {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: StaticMap<String, String>,
    // pub streams: Streams,
}

#[derive(Debug)]
pub struct Streams {
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
}

impl Default for Streams {
    fn default() -> Self {
        Self {
            stdin: Stdio::inherit(),
            stdout: Stdio::inherit(),
            stderr: Stdio::inherit(),
        }
    }
}

impl SimpleCmd {
    pub fn push_ident(&mut self, ident: String) {
        if self.cmd.is_empty() {
            self.cmd = ident;
        } else {
            self.args.push(ident);
        }
    }

    pub fn build(self) -> Cmd {
        if self.cmd.is_empty() {
            Cmd::Empty
        } else {
            Cmd::Simple(self)
        }
    }
}

// let res = unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) };

// use crate::helpers::{Fd, Shell};
// use crate::lexer::Lexer;
// use crate::lexer::Token::{self, *};
// use crate::lexer::{
//     Action,
//     Expand::{self, *},
//     Op,
// };
// use nix::unistd::User;
// use os_pipe::pipe;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::env;
// use std::io::Write;
// use std::iter::Peekable;
// use std::process::exit;
// use std::rc::Rc;
// use crate::runner::Runner;
//
//

//
// // Keeps track of io in one spot before it's put into a command
// pub struct Io {
//     stdin: Rc<RefCell<Fd>>,
//     stdout: Rc<RefCell<Fd>>,
//     stderr: Rc<RefCell<Fd>>,
// }
//
// impl Io {
//     fn new() -> Io {
//         Io {
//             stdin: Rc::new(RefCell::new(Fd::Stdin)),
//             stdout: Rc::new(RefCell::new(Fd::Stdout)),
//             stderr: Rc::new(RefCell::new(Fd::Stderr)),
//         }
//     }
//
//     fn set_stdin(&mut self, fd: Rc<RefCell<Fd>>) {
//         self.stdin = fd;
//     }
//
//     fn set_stdout(&mut self, fd: Rc<RefCell<Fd>>) {
//         self.stdout = fd;
//     }
//
//     fn set_stderr(&mut self, fd: Rc<RefCell<Fd>>) {
//         self.stderr = fd;
//     }
// }
//

//
// // The parser struct. Keeps track of current location in a peekable iter of tokens
// pub struct Parser {
//     shell: Shell,
//     lexer: Lexer,
// }
//
// impl Parser {
//     pub fn new(lexer: Lexer, shell: Shell) -> Parser {
//         Parser { shell, lexer }
//     }
//
//     pub fn get(&mut self) -> Result<Cmd, String> {
//         let mut node = self.get_pipe()?;
//         while let Some(Op(Op::And)) | Some(Op(Op::Or)) = self.lexer.peek() {
//             if let Some(Op(Op::And)) = self.lexer.next_token(&mut self.shell) {
//                 node = Cmd::And(Box::new(node), Box::new(self.get_pipe()?));
//             } else {
//                 node = Cmd::Or(Box::new(node), Box::new(self.get_pipe()?));
//             }
//         }
//         Ok(node)
//     }
//
//     pub fn get_pipe(&mut self) -> Result<Cmd, String> {
//         let mut node = self.get_simple()?;
//         while let Some(Op(Op::Pipe)) = self.lexer.peek() {
//             self.lexer.next();
//             node = Cmd::Pipeline(Box::new(node), Box::new(self.get_simple()?));
//         }
//         Ok(node)
//     }
//
//     pub fn get_simple(&mut self) -> Result<Cmd, String> {
//         if let Some(Op(Op::Bang)) = self.lexer.peek() {
//             self.lexer.next();
//             Ok(Cmd::Not(Box::new(self.get_simple()?)))
//         } else {
//             let mut result = Vec::new();
//             let mut io = Io::new();
//             let mut map = HashMap::new();
//
//             loop {
//                 match self.lexer.peek() {
//                     Some(Word(_)) => {
//                         if let Some(Word(mut expansions)) = self.lexer.next() {
//                             if let [Literal(_)] = &expansions[..] {
//                                 result.push(expansions.pop().unwrap().get_name())
//                             } else {
//                                 let word = self.expand_word(expansions);
//                                 if !word.is_empty() {
//                                     result.push(word)
//                                 }
//                             }
//                         }
//                     }
//                     Some(Assign(_, _)) => {
//                         if let Some(Assign(key, var)) = self.lexer.next() {
//                             map.insert(key, self.expand_word(var));
//                         }
//                     }
//                     Some(Op(Op::Less)) => {
//                         self.lexer.next_token(&mut self.shell);
//                         io.set_stdin(self.token_to_fd(&io)?);
//                     }
//                     Some(Op(Op::More)) => {
//                         self.lexer.next_token(&mut self.shell);
//                         io.set_stdout(self.token_to_fd(&io)?);
//                     }
//                     Some(Integer(_)) => {
//                         if let Some(Integer(int)) = self.lexer.next() {
//                             if let Some(Op(_)) = self.lexer.peek() {
//                                 self.lexer.next();
//                                 match int {
//                                     0 => io.set_stdin(self.token_to_fd(&io)?),
//                                     1 => io.set_stdout(self.token_to_fd(&io)?),
//                                     2 => io.set_stderr(self.token_to_fd(&io)?),
//                                     _ => todo!(),
//                                 }
//                             } else {
//                                 result.push(int.to_string());
//                             }
//                         }
//                     }
//                     _ => break,
//                 }
//             }
//             if result.is_empty() {
//                 if map.is_empty() {
//                     Err(String::from("rush: expected command but found none"))
//                 } else {
//                     map = map
//                         .into_iter()
//                         .filter_map(|(k, v)| {
//                             if env::var_os(&k).is_some() {
//                                 env::set_var(k, v);
//                                 None
//                             } else {
//                                 Some((k, v))
//                             }
//                         })
//                         .collect();
//                     self.shell.borrow_mut().vars.extend(map);
//                     Ok(Cmd::Empty)
//                 }
//             } else {
//                 let mut cmd = Simple::new(result.remove(0), result, io);
//                 if !map.is_empty() {
//                     cmd.add_env(map);
//                 }
//                 Ok(Cmd::Simple(cmd))
//             }
//         }
//     }
//
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
//
//     fn token_to_fd(&mut self, io: &Io) -> Result<Rc<RefCell<Fd>>, String> {
//         let error = String::from("rush: expected redirection location but found none");
//         if let Some(token) = self.lexer.next() {
//             match token {
//                 Op(Op::Ampersand) => {
//                     if let Some(Integer(i)) = self.lexer.next() {
//                         Ok(Rc::clone(match i {
//                             0 => &io.stdin,
//
