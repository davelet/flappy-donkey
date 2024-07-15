use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop};
use bracket_lib::terminal::GameState;

fn main() -> BError{
    let c = BTermBuilder::simple80x50().with_title("Flappy Donkey").build()?;
    main_loop(c, State {y:0})
}
struct State {
    y: i32
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.y = self.y +1;
        ctx.cls();
        ctx.print(1, self.y, "Hello bterm");
    }
}