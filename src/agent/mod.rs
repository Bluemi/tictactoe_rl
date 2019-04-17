pub mod random_agent;
pub mod interactive_agent;
pub mod tictactoe_agent;

use crate::logic::State;

pub type GameMove = u32;

pub trait Agent {
    fn next_move(&mut self, state: State) -> GameMove;
    fn apply_reward(&mut self, _reward: f64) {}
    fn reset(&mut self) {}
}
