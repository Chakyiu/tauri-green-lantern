#[derive(Clone, Copy, Debug)]
pub struct AppState {
    pub idle_time: u64,
}

impl AppState {
    pub fn new(idle_in_sec: u64) -> Self {
        Self {
            idle_time: idle_in_sec,
        }
    }
}
