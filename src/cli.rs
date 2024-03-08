use std::{env, path::PathBuf};

use clap::Parser;

pub enum ShellMode {
    Run(PathBuf),
    /// run code from stdin
    Eval,
    Interactive,
    Login,
    Command(String),
}

impl ShellMode {
    pub fn get() -> Self {
        Self::from(Args::parse())
    }
}

impl From<Args> for ShellMode {
    fn from(value: Args) -> Self {
        if ({ env::args().next().is_some_and(|name| name.starts_with('-')) } || value.login) && {
            // TODO: check if there is already a login shell
            true
        } {
            // if (this was started with the login shell prefix or it is
            // explicitally a login shell) and there is not already a login shell
            Self::Login
        } else if value.interactive {
            // if we are said to be interactive the we are
            Self::Interactive
        } else if let Some(cmd) = value.command {
            // if we have an input command then run that
            Self::Command(cmd)
        } else if let Some(file) = value.file {
            // if we have and input file then run that
            Self::Run(PathBuf::from(file))
        } else if atty::is(atty::Stream::Stdin) {
            // if there is no input file and stdin is a tty then we are running
            // iteractivally
            Self::Interactive
        } else {
            // if there is no input file and stdin is not a tty then we should
            // run code we receive. This likely means we are in a pipeline
            Self::Eval
        }
    }
}
/// A shell to fixx the world. Kinda... not.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to run.
    #[arg(short, long)]
    command: Option<String>,

    /// Is this a login shell.
    #[arg(short, long)]
    login: bool,

    /// Is this an interactive shell.
    #[arg(short, long)]
    interactive: bool,

    /// File to execute. Pass `-` to read stdin, which is default
    file: Option<String>,
}
