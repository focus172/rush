mod cd;
mod exit;

use std::sync::Arc;

pub use cd::Cd;
pub use exit::Exit;

use super::state::ShellState;

pub trait ShellBuiltin {
    fn run(
        args: Box<[String]>,
        state: Arc<ShellState>,
    ) -> impl std::future::Future<Output = i32> + Send;

    fn call(args: Box<[String]>, state: Arc<ShellState>) -> tokio::task::JoinHandle<i32> {
        tokio::spawn(async { Self::run(args, state).await })
    }
}
