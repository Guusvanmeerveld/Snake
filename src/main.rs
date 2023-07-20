use constants::SCALE;
use game::{Game, Options};
use ggez::{event, ContextBuilder};

mod constants;
mod direction;
mod error;
mod game;
mod location;
mod object;
mod world;

fn main() {
    // Make a Context.
    let options = Options::default();

    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            (options.world().width() * SCALE) as f32,
            (options.world().height() * SCALE) as f32,
        ))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = Game::new(options).expect("Failed to create game");

    game.start();

    // Run!
    event::run(ctx, event_loop, game);
}
