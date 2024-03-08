use crate::shell::state::ShellState;

pub fn cd(args: Vec<String>, state: &mut ShellState) -> i32 {
    let _ = state;
    let dir = args
        .into_iter()
        .next()
        .unwrap_or_else(|| state.home().to_owned());

    // libc::chdir(dir)
    std::env::set_current_dir(dir).unwrap();
    0
}
