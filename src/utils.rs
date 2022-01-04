use std::ops;

use crate::wasm4::*;
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i8,
    pub y: i8,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        let mut vec = self;
        vec.x += rhs.x;
        vec.y += rhs.y;
        vec
    }
}

pub fn set_draw_color(color: u16) {
    unsafe {
        *DRAW_COLORS = color;
    }
}

pub fn draw_cell(pos: Vec2) {
    rect(
        (pos.x * (CELL_SIZE as i8)) as i32,
        (pos.y * (CELL_SIZE as i8)) as i32,
        CELL_SIZE,
        CELL_SIZE,
    );
}
