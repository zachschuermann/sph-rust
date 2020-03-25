//! The simplest possible example that does something.
//! changing to do sph simulation

use ggez;
use ggez::event;
//use ggez::graphics;
//use ggez::nalgebra as na;
use ggez::GameResult;//{Context, GameResult};

use sph;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut sph::State::new()?;
    event::run(ctx, event_loop, state)
}
