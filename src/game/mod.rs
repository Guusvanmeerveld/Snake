use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    input::keyboard::KeyInput,
    Context, GameResult,
};

use crate::{constants::DESIRED_FPS, direction::Direction, error::Result, world::World};

pub use self::options::{Options, WorldOptions};

mod options;

pub struct Game {
    world: World,
    running: bool,
    options: Options,
}

impl Game {
    pub fn new(options: Options) -> Result<Self> {
        let seed: u64 = options.seed().parse()?;

        let game = Self {
            world: World::new(seed, options.world()),
            running: false,
            options: Options::default(),
        };

        Ok(game)
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    fn step(&mut self) {
        let snake = self.world.snake();

        if self.world.on_food() {
            self.world.snake_mut().add_body();

            self.world.eat_food();
        } else {
            if let Some(body) = snake.body() {
                body.borrow_mut().follow(snake.direction())
            }
        }

        if !self.world.move_snake() {
            self.running = false;
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if self.running {
                self.step();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.world.snake().draw(&mut canvas);
        self.world.food().draw(&mut canvas);
        // Draw code here...
        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(direction) = input.keycode.and_then(Direction::from_keycode) {
            self.world.snake_mut().set_direction(direction)
        }
        Ok(())
    }
}
