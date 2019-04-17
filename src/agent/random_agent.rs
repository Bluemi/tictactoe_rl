use crate::random::RandomGenerator;
use crate::agent::{Agent, GameMove};
use crate::logic::{State, get_random_state_position};

pub struct RandomAgent {
    random_generator: RandomGenerator,
}

impl RandomAgent {
    pub fn new() -> RandomAgent {
        RandomAgent { random_generator: RandomGenerator::new() }
    }
}

impl Agent for RandomAgent {
    fn next_move(&mut self, state: State) -> GameMove {
        return get_random_state_position(state, &mut self.random_generator);
    }
}
