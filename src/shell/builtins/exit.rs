use crate::shell::state::ShellState;

pub fn exit(args: Vec<String>, state: &mut ShellState) -> i32 {
    let _ = args;
    eprintln!("exit");
    state.exit = true;
    0
}
