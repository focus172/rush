#[derive(Debug)]
pub struct ShellState {
    pub exit: bool,
    home: String,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            exit: false,
            home: std::env::var("HOME").unwrap(),
        }
    }
}
impl ShellState {
    pub fn home(&self) -> &str {
        &self.home
    }
}
