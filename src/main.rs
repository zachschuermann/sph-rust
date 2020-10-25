//! The simplest possible example that does something.
//! changing to do sph simulation

use ggez;
use ggez::event;
use ggez::GameResult;

use sph;

pub fn main() -> GameResult {
    println!("Running simulation...");
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut sph::State::new(1200)?;
    event::run(ctx, event_loop, state)
}
