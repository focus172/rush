use std::os::fd::FromRawFd;
use std::process::Stdio;
use std::sync::Arc;

use crate::parse::cmd::Cmd;
use crate::parse::cmd::SimpleCmd;
use crate::parse::cmd::Streams;
use crate::prelude::*;
use crate::shell::builtins;

use super::state::ShellState;

use crate::shell::task::Task;

#[derive(Debug, Default)]
pub struct Driver;

#[derive(Debug)]
pub enum DriverError {
    Spawn,
    Pipe,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::Spawn => f.write_str("failed to spawn command"),
            DriverError::Pipe => f.write_str("failed to open pipe"),
        }
    }
}
impl Context for DriverError {}

use crate::shell::builtins::ShellBuiltin;

impl Driver {
    pub fn run(
        cmd: Cmd,
        streams: Streams,
        state: &mut Arc<ShellState>,
    ) -> Result<Vec<Task>, DriverError> {
        match cmd {
            Cmd::Simple(SimpleCmd { cmd, args, env }) => match cmd.as_str() {
                "exit" => Ok(vec![Task::Builtin(builtins::Exit::call(
                    args.into_boxed_slice(),
                    state.clone(),
                ))]),
                "cd" => Ok(vec![Task::Builtin(builtins::Cd::call(
                    args.into_boxed_slice(),
                    state.clone(),
                ))]),
                cmd => {
                    log::info!("Running command: [{}]", cmd);

                    let child = tokio::process::Command::new(cmd)
                        .args(args)
                        .envs(env)
                        .stdout(streams.stdout)
                        .stdin(streams.stdin)
                        .stderr(streams.stderr)
                        .spawn()
                        .change_context(DriverError::Spawn)?;

                    // for handling things like ^C and ^Z
                    // let stdin = child.stdin.take().unwrap();

                    Ok(vec![Task::from(child)])
                }
            },
            Cmd::Pipeline(c, d) => {
                let mut pipes = [0; 2];

                if unsafe { libc::pipe(pipes.as_mut_ptr()) } != 0 {
                    let err = Err(std::io::Error::last_os_error());
                    return err.change_context(DriverError::Pipe);
                }

                let sc = Streams {
                    stdout: unsafe { Stdio::from_raw_fd(pipes[1]) },
                    ..Default::default()
                };

                let mut sd = streams;
                sd.stdin = unsafe { Stdio::from_raw_fd(pipes[0]) };

                let mut a = Self::run(*c, sc, state)?;
                let b = Self::run(*d, sd, state)?;

                a.extend(b);
                Ok(a)
            }
            Cmd::And(_, _) => todo!(),
            Cmd::Or(_, _) => todo!(),
            Cmd::Not(_) => todo!(),
            Cmd::Empty => Ok(vec![]),
        }
    }
}
