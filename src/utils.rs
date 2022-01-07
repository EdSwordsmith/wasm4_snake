use core::panic::PanicInfo;

use crate::wasm4::*;
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i8,
    pub y: i8,
}

pub fn set_draw_color(color: u16) {
    unsafe {
        *DRAW_COLORS = color;
    }
}

pub fn draw_cell(pos: Vec2) {
    let x = ((pos.x as u32) * CELL_SIZE) as i32;
    let y = ((pos.y as u32) * CELL_SIZE) as i32;
    rect(x, y, CELL_SIZE, CELL_SIZE);
}

static mut RAND_SEED: u64 = 0xFF;

pub fn rand() -> u64 {
    unsafe {
        RAND_SEED ^= RAND_SEED << 13;
        RAND_SEED ^= RAND_SEED >> 17;
        RAND_SEED ^= RAND_SEED << 5;

        return RAND_SEED;
    }
}

pub fn randrange(n: u64) -> i32 {
    return (rand() % n) as i32;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
