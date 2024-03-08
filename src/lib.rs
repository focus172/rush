#![feature(let_chains, yeet_expr)]

pub mod cli;
pub mod lexer;
pub mod parse;
pub mod prelude;
pub mod shell;
pub mod util;

use crate::prelude::*;
use std::{fs, process::ExitCode};

use crate::{
    cli::ShellMode,
    lexer::Lexer,
    parse::{Parser, Prompter},
};

pub fn rush() -> ExitCode {
    self::logger();

    let exit = match ShellMode::get() {
        ShellMode::Run(path) => {
            log::info!("running file: {:?}", path);

            let data = fs::read_to_string(path).unwrap();
            let input = data.chars().peekable();
            let shell = Shell::eval(Parser::new(Lexer::new(input)));
            shell.run()
        }
        ShellMode::Eval => todo!(),
        ShellMode::Interactive => {
            log::info!("running interactive session");
            Shell::interactive(Prompter::new()).run()
        }
        ShellMode::Login => {
            log::info!("running login session");

            Shell::login(Prompter::new()).run()
        }
        ShellMode::Command(cmd) => {
            log::info!("running command: {:?}", cmd);

            let input = cmd.chars().peekable();
            let shell = Shell::eval(Parser::new(Lexer::new(input)));

            shell.run()
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
    simplelog::TermLogger::init(
        log::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();
}
