use std::sync::{atomic::Ordering, Arc};

use crate::shell::state::ShellState;

use super::ShellBuiltin;

async fn exit(args: &[String], state: Arc<ShellState>) -> i32 {
    let _ = args;
    eprintln!("exit");

    state.exit.store(true, Ordering::Relaxed);
    0
}

pub struct Exit;
impl ShellBuiltin for Exit {
    async fn run(args: Box<[String]>, state: Arc<ShellState>) -> i32 {
        self::exit(&args, state).await
    }
}
