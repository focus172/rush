use std::sync::atomic::AtomicBool;

use crate::util::smap::StaticMap;

#[derive(Debug)]
pub struct ShellState {
    pub exit: AtomicBool,
    pub envs: StaticMap<String, String>,
    home: String,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            exit: AtomicBool::new(false),
            envs: StaticMap::default(),
            home: std::env::var("HOME").unwrap(),
        }
    }
}
impl ShellState {
    pub fn home(&self) -> &str {
        &self.home
    }
}
