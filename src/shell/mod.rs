use crate::prelude::*;

mod driver;

// use nix::unistd::Uid;
// use os_pipe::{dup_stderr, dup_stdin, dup_stdout, PipeReader, PipeWriter};
// use std::env;
// use std::fs::{File, OpenOptions};
// use std::io::{self, BufRead, BufReader, Write};
// use std::process::{self, Stdio};

use crate::parse::cmd::{Cmd, CmdError};

use self::driver::Driver;

#[derive(Debug)]
pub enum ShellError {
    ParseError,
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::ParseError => f.write_str("failed to parse input"),
        }
    }
}
impl Context for ShellError {}

/// The shell works through a cycle of getting some tokens. Collecting them
/// into a Command. Then running it.
pub struct Shell<I>
where
    I: Iterator<Item = Result<Cmd, CmdError>>,
{
    cmds: I,
    driver: Driver,
    interactive: bool,
}

impl<I> Shell<I>
where
    I: Iterator<Item = Result<Cmd, CmdError>>,
{
    /// Contructs a new shell in command mode. The given input will be run until
    /// exhaustion then the shell will exit.
    ///
    /// This argument is ussaually a [`Lexer`] but is generic so it can take
    /// many types of lexers.
    pub fn eval(cmds: I) -> Shell<I> {
        Shell {
            cmds,
            interactive: false,
            driver: Driver::default(),
        }
    }

    pub fn interactive(cmds: I) -> Shell<I> {
        Shell {
            cmds,
            interactive: true,
            driver: Driver::default(),
        }
    }

    pub fn login(_cmds: I) -> Shell<I> {
        panic!("I'm not ready for this shit yet. Try next year.")
    }

    /// Runs the main event loop for this shell. Gets commands from the its
    /// stream and evaluates them.
    ///
    /// # Errors
    /// When this function shell is interactive (almost) all error are treated
    /// non-fatally. A message is printed to the user and the shell continues
    /// as normal. When running non-interactively the command this will return
    /// and error.
    ///
    /// ## Login
    /// When this is ran as a login shell it will refuse to panic or error.
    /// The shell will attempt to restart itself whenever some thing bad
    /// happens.
    pub fn run(self) -> Result<(), ShellError> {
        let mut shell = self;
        for res in shell.cmds {
            let cmd = if shell.interactive {
                res.change_context(ShellError::ParseError)?
            } else {
                match res {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        continue;
                    }
                }
            };

            shell.driver.run(cmd).unwrap();
        }

        Ok(())
    }

    fn is_interactive(&self) -> bool {
        self.interactive
    }

    // pub fn next_prompt(&mut self, prompt: &str) -> Option<String> {
    //     if self.is_interactive() {
    //         print!("{}", prompt);
    //         io::stdout().flush().unwrap();
    //     }
    //     self.lines.next()
    // }

    // // Not super satisfied with this as it is returning a String when it could be a
    // // reference, but this also allows handling stuff like $@ right here, as that would need to be
    // // stitched together here and thus it would own the value.
    // // Also, env:: calls in Rust seem to return ownership rather than references, which is
    // // nasty.
    // pub fn get_var(&self, key: &str) -> Option<String> {
    //     if let Ok(num) = key.parse::<u32>() {
    //         if num == 0 {
    //             Some(self.name.clone())
    //         } else {
    //             self.get_pos(num).map(String::from)
    //         }
    //     } else {
    //         match key {
    //             "@" | "*" => Some(self.positional.join(" ")), // these are technically more complicated but it works for now
    //             "#" => Some(self.positional.len().to_string()),
    //             "$" => Some(process::id().to_string()),
    //             _ => self
    //                 .vars
    //                 .get(key)
    //                 .map_or(env::var(key).ok(), |s| Some(String::from(s))),
    //         }
    //     }
    // }
    //
    // pub fn set_var(&mut self, key: String, val: String) {
    //     if env::var_os(&key).is_some() {
    //         env::set_var(key, val);
    //     } else {
    //         self.vars.insert(key, val);
    //     }
    // }
}

// impl Iterator for Shell {
//     type Item = String;
//
//     fn next(&mut self) -> Option<String> {
//         if self.is_interactive() {
//             if Uid::current().is_root() {
//                 print!("#> ");
//             } else {
//                 print!("$> ");
//             }
//             io::stdout().flush().unwrap();
//         }
//         self.lines.next()
//     }
// }
//
