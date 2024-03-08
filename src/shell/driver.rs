use std::io::Write;

use crate::parse::cmd::SimpleCmd;
use crate::prelude::*;
use crate::{parse::cmd::Cmd, util::smap::StaticMap};

#[derive(Debug, Default)]
pub struct Driver {
    envs: StaticMap<String, String>,
}

#[derive(Debug)]
pub enum DriverError {
    Fuck,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::Fuck => todo!(),
        }
    }
}
impl Context for DriverError {}

impl Driver {
    pub fn run(&mut self, cmd: Cmd) -> Result<(), DriverError> {
        match cmd {
            Cmd::Simple(SimpleCmd {
                cmd,
                args,
                env,
                streams,
            }) => {
                log::info!("Running command: [{}]", cmd);
                let mut c = std::process::Command::new(cmd)
                    .args(args)
                    .envs(env)
                    .stdout(std::process::Stdio::inherit())
                    .stdin(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .spawn()
                    .unwrap();

                c.wait().unwrap();

                Ok(())
            }
            Cmd::Pipeline(_, _) => todo!(),
            Cmd::And(_, _) => todo!(),
            Cmd::Or(_, _) => todo!(),
            Cmd::Not(_) => todo!(),
            Cmd::Empty => Ok(()),
        }
    }
}
