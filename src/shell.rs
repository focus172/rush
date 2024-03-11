use crate::parse::{Parser, Prompter};
use crate::prelude::*;

// use nix::unistd::Uid;
// use os_pipe::{dup_stderr, dup_stdin, dup_stdout, PipeReader, PipeWriter};
// use std::env;
// use std::fs::{File, OpenOptions};
// use std::io::{self, BufRead, BufReader, Write};
// use std::process::{self, Stdio};

use crate::parse::{Cmd, CmdError, Streams};

use crate::drive::Driver;

// use crate::util::StaticMap;

#[derive(Debug)]
pub(crate) struct ShellState {
    pub exit: bool,
    home: String,
    // __cache: StaticMap<String, String>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            exit: false,
            home: std::env::var("HOME").unwrap(),
            // __cache: StaticMap::new()
        }
    }
}
impl ShellState {
    pub fn home(&self) -> &str {
        &self.home
    }

    pub fn get_env_exact(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    /// Gets an variable from the current scope. This matches on the smallest
    /// substring first. it returns the rest of the unmatched slice as the
    /// second. if the variable doesn't exist it will return ("", key).
    pub fn get_env<'a>(&'a self, key: &'a str) -> (String, &'a str) {
        for i in 0..key.len() {
            log!("trying key: {}", &key[0..i + 1]);
            if let Some(v) = self.get_env_exact(&key[0..i + 1]) {
                return (v, &key[i + 1..]);
            }
        }
        (String::new(), key)
    }
}

#[derive(Debug)]
pub enum ShellError {
    Parse,
    Spawn,
    Task,
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Parse => f.write_str("failed to parse input"),
            ShellError::Spawn => f.write_str("failed spawn thing"),
            ShellError::Task => f.write_str("background task had a problem"),
        }
    }
}
impl Context for ShellError {}

pub(crate) enum CommandSource<I>
where
    I: Iterator<Item = Token>,
{
    Interactive(Prompter),
    NonInteractively(Parser<I>),
}

impl<I> CommandSource<I>
where
    I: Iterator<Item = Token>,
{
    pub fn next(&mut self, state: &ShellState) -> Option<Result<Cmd, CmdError>> {
        match self {
            CommandSource::Interactive(i) => i.next(state),
            CommandSource::NonInteractively(s) => s.next(state),
        }
    }
}

/// The shell works through a cycle of getting some tokens. Collecting them
/// into a Command. Then running it.
pub struct Shell<I>
where
    I: Iterator<Item = Token>,
{
    cmmds: CommandSource<I>,
    state: ShellState,
    // taskp: Vec<Task>,
}
impl Shell<std::iter::Empty<Token>> {
    pub fn interactive() -> Shell<std::iter::Empty<Token>> {
        Shell {
            cmmds: CommandSource::Interactive(Prompter::default()),
            state: ShellState::default(),
        }
    }

    pub fn login() -> Shell<std::iter::Empty<Token>> {
        panic!("I'm not ready for this shit yet. Try next year.")
    }
}

impl<I> Shell<I>
where
    I: Iterator<Item = Token>,
{
    /// Contructs a new shell in command mode. The given input will be run until
    /// exhaustion then the shell will exit.
    ///
    /// This argument is ussaually a [`Lexer`] but is generic so it can take
    /// many types of lexers.
    pub fn sourced(tokens: I) -> Shell<I> {
        Shell {
            cmmds: CommandSource::NonInteractively(Parser::new(tokens)),
            state: ShellState::default(),
        }
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
    pub fn run(mut self, interactive: bool) -> Result<(), ShellError> {
        // let mut hand = Vec::new();

        while let Some(res) = self.cmmds.next(&self.state) {
            // let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(16);

            let cmd = {
                match (res, interactive) {
                    (Ok(cmd), _) => cmd,
                    (Err(e), true) => {
                        eprintln!("{:?}", e);
                        continue;
                    }
                    (Err(e), false) => return Err(e.change_context(ShellError::Parse)),
                }
            };

            let res = Driver::run(cmd, Streams::default(), &mut self.state);

            let handles = match (res, interactive) {
                (Ok(a), _) => a,
                (Err(e), true) => {
                    eprintln!("{:?}", e);
                    continue;
                }
                (Err(e), false) => return Err(e.change_context(ShellError::Spawn)),
            };

            for h in handles {
                let _ = h
                    .wait()
                    .change_context(ShellError::Spawn)
                    .attach("task had internal error")?;
            }

            // while let Some(a) = rx.recv().await {
            //     read += 1;
            //     eprintln!("process had status: {}", a);
            //
            //     if read >= count {
            //         break;
            //     }
            // }

            if self.state.exit {
                log!("force exiting.");
                return Ok(());
            }
        }
        log!("no more commands.");

        Ok(())
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