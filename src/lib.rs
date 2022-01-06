#![no_std]

mod wasm4;
mod snake;
mod utils;

use core::str::from_utf8_unchecked;
use wasm4::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::snake::*;
use crate::utils::*;

const CELL_SIZE: u32 = 4;
const GAME_SIZE: u32 = SCREEN_SIZE / CELL_SIZE;
const MAX_SIZE: usize = (GAME_SIZE * GAME_SIZE) as usize;
const FPS: u8 = 5;

static mut SNAKE: Snake = new_snake!();
static mut FRUIT: Vec2 = Vec2 { x: 20, y: 20 };
static mut RNG: Option<Pcg32> = None;

#[no_mangle]
fn start() {
    let rng = unsafe { &mut RNG };
    *rng = Some(Pcg32::seed_from_u64(0));
    unsafe {
        *PALETTE = [0xe5dcc8, 0x0b7a75, 0x19535f, 0x7b2d26];
    }
}

#[no_mangle]
fn update() {
    let snake = unsafe { &mut SNAKE };
    let fruit = unsafe { &mut FRUIT };

    snake.tick();
    snake.draw();

    // Draw score
    let mut buffer = [0_u8; 4];
    let mut score = snake.size - 1;
    for byte in buffer.iter_mut().rev() {
        *byte = 48 + (score % 10) as u8;
        score = score / 10;
    }
    let score_text = unsafe { from_utf8_unchecked(&buffer) };
    set_draw_color(3);
    text(score_text, 1, 1);

    // draw fruit
    set_draw_color(4);
    draw_cell(*fruit);
}
