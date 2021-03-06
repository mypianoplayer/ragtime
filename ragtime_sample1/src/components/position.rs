extern crate ragtime;
use std::ptr;
use components::input;
use protocol::Protocol;


#[derive(Default)]
pub struct Position {
    pos: (f32,f32),
}

impl Position {
    pub fn new() -> Position {
        Position {..Default::default()}
    }
    pub fn start(&mut self) {}
    pub fn update(&mut self, protocols: &mut Vec<Protocol>, speed: (f32,f32)) {
        self.pos.0 += speed.0;
        self.pos.1 += speed.1;
    }
}

impl Position {
    pub fn move_to(&mut self, tgt_pos: (f32, f32), after_msec: i32) {}
}
