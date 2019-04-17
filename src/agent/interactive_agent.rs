use crate::agent::{GameMove, Agent};
use crate::logic::{State, get_interactive_state_position};

pub struct InteractiveAgent { }

impl InteractiveAgent {
    pub fn new() -> InteractiveAgent {
        InteractiveAgent { }
    }
}

impl Agent for InteractiveAgent {
    fn next_move(&mut self, state: State) -> GameMove {
        get_interactive_state_position(state)
    }

    fn apply_reward(&mut self, reward: f64) {
        println!("reward: {}", reward);
    }
}
