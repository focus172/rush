#![feature(let_chains, yeet_expr)]

pub mod cli;
pub mod lexer;
pub mod parse;
pub mod prelude;
pub mod shell;
pub mod util;

use crate::prelude::*;
use std::{fs, io::Read, process::ExitCode};

use crate::{
    cli::ShellMode,
    lexer::Lexer,
    parse::{Parser, Prompter},
};

#[tokio::main(flavor = "current_thread")]
pub async fn rush() -> ExitCode {
    self::logger();

    let exit = match ShellMode::get() {
        ShellMode::Run(path) => {
            log::info!("running file: {:?}", path);

            let data = fs::read_to_string(path).unwrap();
            let input = data.chars().peekable();

            Shell::new(Parser::new(Lexer::new(input))).run(false).await
        }
        ShellMode::Eval => {
            let mut a = std::io::stdin();

            log::warn!("the implementation for reading from stdin is shitty.");
            let mut buf = String::new();
            a.read_to_string(&mut buf).unwrap();

            let input = buf.chars().peekable();

            Shell::new(Parser::new(Lexer::new(input))).run(false).await
        }
        ShellMode::Interactive => {
            log::info!("running interactive session");

            Shell::new(Prompter::default()).run(true).await
        }
        ShellMode::Login => {
            log::info!("running login session");

            Shell::login(Prompter::default()).run(true).await
        }
        ShellMode::Command(cmd) => {
            log::info!("running command: {:?}", cmd);

            let input = cmd.chars().peekable();

            Shell::new(Parser::new(Lexer::new(input))).run(false).await
        }
    };

    match exit {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprint!("{:?}", e);
            // TODO: more complext exit codes
            ExitCode::FAILURE
        }
    }
}

fn logger() {
    if std::env::var("LOG").is_ok() {
        simplelog::TermLogger::init(
            log::LevelFilter::Trace,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        )
        .unwrap();
    }
}
