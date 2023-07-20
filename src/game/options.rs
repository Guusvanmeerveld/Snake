use std::time::Duration;

use rand::Rng;

pub struct Options {
    seed: String,
    world: WorldOptions,
    speed: Duration,
}

impl Options {
    pub fn world(&self) -> &WorldOptions {
        &self.world
    }

    pub fn speed(&self) -> Duration {
        self.speed
    }

    pub fn seed(&self) -> &str {
        self.seed.as_ref()
    }
}

pub struct WorldOptions {
    height: isize,
    width: isize,
}

impl WorldOptions {
    pub fn height(&self) -> isize {
        self.height
    }

    pub fn width(&self) -> isize {
        self.width
    }
}

impl Default for WorldOptions {
    fn default() -> Self {
        let height = 20;
        let width = 20;

        Self { height, width }
    }
}

impl Default for Options {
    fn default() -> Self {
        let seed: u64 = rand::thread_rng().gen();

        Self {
            seed: seed.to_string(),
            speed: Duration::from_millis(200),
            world: WorldOptions::default(),
        }
    }
}
