use crate::logic::{State, StatePosition, get_random_state_position, get_possible_state_positions, EntryKind};
use crate::random::RandomGenerator;
use crate::agent::{Agent, GameMove};
use crate::logic;

const STEP_SIZE_DECAY: f64 = 0.99999;
// const STEP_SIZE_DECAY: f64 = 1.0;
const START_STEP_SIZE: f64 = 0.1;
const NUM_STATES: usize = (1 << 18) -1;
const START_VALUE: f64 = 0.0;

pub struct TicTacToeAgent {
    state_values: Vec<f64>,
    current_states: [State; 5],
    num_current_states: usize,
    random_generator: RandomGenerator,
    step_size: f64,
    entry_kind_shift_width: u32,
    exploit: bool,
    debug: bool,
}

impl TicTacToeAgent {
    pub fn new(entry_kind: EntryKind) -> TicTacToeAgent {
        TicTacToeAgent {
            state_values: vec![START_VALUE; NUM_STATES],
            current_states: [0; 5],
            num_current_states: 0,
            random_generator: RandomGenerator::new(),
            step_size: START_STEP_SIZE,
            entry_kind_shift_width: entry_kind.get_shift_size(),
            exploit: false,
            debug: false,
        }
    }

    pub fn exploit(&mut self) {
        self.exploit = true;
    }

    pub fn debug(&mut self) {
        self.debug = true;
    }

    fn next_exploit_move(&mut self, state: State) -> GameMove {
        let mut possible_moves: [u32; 9] = [0; 9];
        let num_possible_moves = get_possible_state_positions(&mut possible_moves, state);

        if num_possible_moves == 0 { return 0; }

        let mut best_move = possible_moves[0];
        let best_next_state = state | (best_move << self.entry_kind_shift_width);
        let mut best_value = self.state_values[best_next_state as usize];

        for i in 0..num_possible_moves {
            let possible_move = possible_moves[i as usize];
            let possible_state = state | (possible_move << self.entry_kind_shift_width);
            let possible_value = self.state_values[possible_state as usize];
            if possible_value > best_value {
                best_move = possible_move;
                best_value = possible_value;
            }
        }

        // println!("win probability: {}", best_value);

        return best_move;
    }

    fn next_explore_move(&mut self, state: State) -> GameMove {
        let state_position: StatePosition = get_random_state_position(state,
                                                                      &mut self.random_generator);

        state_position
    }

    fn save_state(&mut self, state: State, next_move: StatePosition) {
        let next_state = state | (next_move << self.entry_kind_shift_width);
        self.current_states[self.num_current_states] = next_state;
        self.num_current_states += 1;

        if self.debug {
            println!("added state {} with value {}:",
                     self.num_current_states,
                     self.state_values[next_state as usize]);
            logic::print_state(next_state);
        }
    }

    pub fn get_number_of_possible_states(&self) -> usize {
        self.state_values.iter().filter(|x| **x != START_VALUE).count()
    }
}

impl Agent for TicTacToeAgent {
    fn next_move(&mut self, state: State) -> GameMove {
        let next_move = if self.exploit {
            self.next_exploit_move(state)
        } else {
            self.next_explore_move(state)
        };

        self.save_state(state, next_move);

        return next_move;
    }

    fn apply_reward(&mut self, reward: f64) {
        if !self.exploit {
            for i in 0..self.num_current_states {
                let state = self.current_states[i] as usize;
                let a = self.state_values[state];
                self.state_values[state] = a + self.step_size * (reward - a);
            }
            self.step_size *= STEP_SIZE_DECAY;
        }
        self.num_current_states = 0;
    }

    fn reset(&mut self) {
        self.num_current_states = 0;
    }
}
