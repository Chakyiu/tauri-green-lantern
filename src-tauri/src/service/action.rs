use enigo::*;

pub struct Action {
    enigo: Enigo,
}

unsafe impl Send for Action {}

impl Action {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn move_mouse(&mut self) {
        self.enigo.mouse_move_relative(1, 1);
        self.enigo.mouse_move_relative(-1, -1);
        dbg!("Mouse moved");
    }
}
