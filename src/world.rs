use rand::{rngs::StdRng, SeedableRng};

use crate::{
    game::WorldOptions,
    location::Location,
    object::{Food, Head, Object},
};

pub struct World {
    rng: StdRng,
    width: isize,
    height: isize,
    snake: Head,
    food: Food,
}

impl World {
    pub fn new(seed: u64, options: &WorldOptions) -> Self {
        let mut rng = SeedableRng::seed_from_u64(seed);

        let head_location = Location::random(&mut rng, 0, 0, options.width(), options.width());
        let food_location = Location::random(&mut rng, 0, 0, options.width(), options.width());

        Self {
            rng,
            height: options.height(),
            width: options.width(),
            snake: Head::new(head_location),
            food: Food::new(food_location),
        }
    }

    pub fn move_snake(&mut self) -> bool {
        self.snake.move_forward(self.width, self.height)
    }

    pub fn on_food(&self) -> bool {
        self.food.same_location(&self.snake)
    }

    pub fn eat_food(&mut self) {
        let new_location = Location::random(&mut self.rng, 0, 0, self.width, self.height);

        if !self.snake.collided(&new_location) {
            self.food.set_location(new_location);
        } else {
            self.eat_food()
        }
    }

    pub fn snake(&self) -> &Head {
        &self.snake
    }

    pub fn snake_mut(&mut self) -> &mut Head {
        &mut self.snake
    }

    pub fn food(&self) -> &Food {
        &self.food
    }
}
