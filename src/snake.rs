use crate::utils::*;
use crate::wasm4::*;
use crate::*;

pub struct Snake {
    pub body: [Vec2; MAX_SIZE],
    pub size: usize,
    pub vel: Vec2,
    pub frames: u8,
}

#[macro_export]
macro_rules! new_snake {
    () => {{
        let mut body = [Vec2 { x: 0, y: 0 }; MAX_SIZE];
        body[0].x = 1;
        Snake {
            body,
            size: 2,
            vel: Vec2 { x: 1, y: 0 },
            frames: 0,
        }
    }};
}

impl Snake {
    fn handle_input(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_RIGHT != 0 && self.vel.y != 0 {
            self.vel.y = 0;
            self.vel.x = 1;
        } else if gamepad & BUTTON_LEFT != 0 && self.vel.y != 0 {
            self.vel.y = 0;
            self.vel.x = -1;
        } else if gamepad & BUTTON_UP != 0 && self.vel.x != 0 {
            self.vel.x = 0;
            self.vel.y = -1;
        } else if gamepad & BUTTON_DOWN != 0 && self.vel.x != 0 {
            self.vel.x = 0;
            self.vel.y = 1;
        }
    }

    fn handle_movement(&mut self) {
        let mut prev = [self.body[0], self.body[0]];
        for i in 1..self.size {
            prev[1] = prev[0];
            prev[0] = self.body[i];
            self.body[i] = prev[1];
        }

        self.body[0] = self.body[0] + self.vel;

        // Wrap movement
        if self.body[0].x >= GAME_SIZE as i8 {
            self.body[0].x = 0;
        } else if self.body[0].x < 0 {
            self.body[0].x = (GAME_SIZE - 1) as i8;
        }

        if self.body[0].y >= GAME_SIZE as i8 {
            self.body[0].y = 0;
        } else if self.body[0].y < 0 {
            self.body[0].y = (GAME_SIZE - 1) as i8;
        }

        // Check for collisions with the body
        for i in 1..self.size {
            if self.body[0] == self.body[i] {
                self.size = 2;
            }
        }
    }

    fn handle_fruit(&mut self) {
        let fruit = unsafe { &mut FRUIT };
        let rng = unsafe { &mut RNG };

        if let Some(rng) = rng {
            if self.body[0] == *fruit {
                self.grow();
                fruit.x = rng.gen_range(0..GAME_SIZE) as i8;
                fruit.y = rng.gen_range(0..GAME_SIZE) as i8;
            }
        }
    }

    fn grow(&mut self) {
        self.body[self.size] = self.body[self.size - 1];
        self.size += 1;
    }

    pub fn tick(&mut self) {
        self.frames = (self.frames + 1) % 60;
        if self.frames % FPS != 0 {
            return;
        }

        self.handle_input();
        self.handle_movement();
        self.handle_fruit();
    }

    pub fn draw(&self) {
        set_draw_color(3);
        draw_cell(self.body[0]);
        set_draw_color(2);
        for i in 1..self.size {
            let cell = self.body[i];
            draw_cell(cell);
        }
    }
}
