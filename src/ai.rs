use crate::game::{State, StatePosition, get_random_state_position, EntryKind};
use crate::random::RandomGenerator;

struct TicTacToeAgent {
    states_values: [f32; 2^18],
    current_states: Vec<State>,
}

impl TicTacToeAgent {
    fn next_move(&mut self, state: State,
                 entry_kind: EntryKind,
                 random_generator: &mut RandomGenerator) -> StatePosition
    {
        let state_position: StatePosition = get_random_state_position(state,
                                                                      entry_kind,
                                                                      random_generator);

        self.current_states.push(state);
        self.current_states.push(state | state_position);

        state_position
    }

    fn apply_reward(&mut self, reward: f32) {
        for state in self.current_states {
            
        }
    }
}