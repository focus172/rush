pub enum Task {
    System(tokio::process::Child),
    Builtin(tokio::task::JoinHandle<i32>),
}

impl From<tokio::process::Child> for Task {
    fn from(value: tokio::process::Child) -> Self {
        Self::System(value)
    }
}

impl From<tokio::task::JoinHandle<i32>> for Task {
    fn from(value: tokio::task::JoinHandle<i32>) -> Self {
        Self::Builtin(value)
    }
}

impl Task {
    pub async fn wait(self) -> i32 {
        log::warn!("this function should be inproved.");
        match self {
            Task::System(mut a) => a.wait().await.unwrap().code().unwrap_or(-127),
            Task::Builtin(a) => a.await.unwrap(),
        }
    }
}

impl Task {
    // pub fn is_done(&self) -> bool {
    //     match self {
    //         Task::System(a) => {
    //             std::thread::sleep(std::time::Duration::from_millis(10));
    //             true
    //         }
    //         Task::Builtin(a) => a.is_none(),
    //     }
    // }

    // pub async fn poll(&mut self) {
    //     match self {
    //         Task::System(_) => todo!(),
    //         Task::Builtin(s) => {
    //             let Some(a) = s.take() else { return };
    //             a.po
    //         }
    //     }
    // }
}
