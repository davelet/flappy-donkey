use std::cmp::max;
use bracket_lib::color::{BLACK, NAVY, RED, YELLOW};
use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop, to_cp437, VirtualKeyCode};
use bracket_lib::terminal::GameState;
use crate::GameMode::{End, Menu, Playing};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
fn main() -> BError {
    let c = BTermBuilder::simple80x50().with_title("Flappy Donkey").build()?;
    main_loop(c, State::new())
}

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            Menu => { self.main_menu(ctx) }
            Playing => { self.play(ctx) }
            End => { self.dead(ctx) }
        }
    }
}


enum GameMode {
    Menu,
    Playing,
    End,
}

impl State {
    fn new() -> Self {
        Self {
            mode: Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0)
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = End;
        }
    }
    fn restart(&mut self) {
        self.mode = Playing;
        self.frame_time = 0.0;
        self.player = Player::new(5, 25)
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => { self.restart() }
                VirtualKeyCode::Q => { ctx.quit() }
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => { self.restart() }
                VirtualKeyCode::Q => { ctx.quit() }
                _ => {}
            }
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        Self {
            x,
            gap_y: 10,
            size: max(2, 20 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in self.gap_y + half_size .. SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let reach_x = self.x == player.x;
        let half_size = self.size / 2;
        let hit_upper = player.y < self.gap_y - half_size;
        let hit_lower = player.y > self.gap_y + half_size;
        reach_x && (hit_lower || hit_upper)
    }
}