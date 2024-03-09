use std::sync::Arc;

use crate::shell::state::ShellState;

use super::ShellBuiltin;

async fn cd(args: Box<[String]>, state: Arc<ShellState>) -> i32 {
    let _ = state;
    let dir = args
        .iter()
        .next()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| state.home().to_owned());

    // a.into_pthread_t()
    // libc::chdir(dir)

    std::env::set_current_dir(dir).unwrap();
    0
}

pub struct Cd;
impl ShellBuiltin for Cd {
    fn run(
        args: Box<[String]>,
        state: std::sync::Arc<ShellState>,
    ) -> impl std::future::Future<Output = i32> + Send {
        self::cd(args, state)
    }
}
