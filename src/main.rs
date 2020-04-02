//! The simplest possible example that does something.
//! changing to do sph simulation

use ggez;
use ggez::event;
use ggez::GameResult;

use sph;

//pub fn main() -> GameResult {
//    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
//    let (ctx, event_loop) = &mut cb.build()?;
//    let state = &mut sph::State::new()?;
//    event::run(ctx, event_loop, state)
//}

fn main() {
    println!("Running simulation...");
    let state = &mut sph::State::new().unwrap();
    for _ in 0..10000 {
        state.update();
    }
}
