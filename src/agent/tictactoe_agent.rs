use crate::logic::{State, StatePosition, get_random_state_position, get_possible_state_positions};
use crate::random::RandomGenerator;
use crate::agent::{Agent, GameMove};

const STEP_SIZE_DECAY: f64 = 0.99999;
// const STEP_SIZE_DECAY: f64 = 1.0;
const START_STEP_SIZE: f64 = 0.1;
const NUM_STATES: usize = (1 << 18) -1;
const START_VALUE: f64 = 0.0;

pub struct TicTacToeAgent {
    state_values: Vec<f64>,
    current_states: [State; 9],
    num_current_states: usize,
    random_generator: RandomGenerator,
    step_size: f64,
    exploit: bool,
}

impl TicTacToeAgent {
    pub fn new() -> TicTacToeAgent {
        TicTacToeAgent {
            state_values: vec![START_VALUE; NUM_STATES],
            current_states: [0; 9],
            num_current_states: 0,
            random_generator: RandomGenerator::new(),
            step_size: START_STEP_SIZE,
            exploit: false,
        }
    }

    pub fn exploit(&mut self) {
        self.exploit = true;
    }

    fn next_exploit_move(&mut self, state: State) -> GameMove {
        let mut possible_moves: [u32; 9] = [0; 9];
        let num_possible_moves = get_possible_state_positions(&mut possible_moves, state);

        if num_possible_moves == 0 { return 0; }

        let mut best_move = possible_moves[0];
        let mut best_value = self.state_values[(state | best_move) as usize];

        for i in 0..num_possible_moves {
            let possible_move = possible_moves[i as usize];
            let possible_state = possible_move | state;
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
        self.current_states[self.num_current_states] = state | next_move;
        self.num_current_states += 1;
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
        for i in 0..self.num_current_states {
            let state = self.current_states[i] as usize;
            let a = self.state_values[state];
            self.state_values[state] = a + self.step_size * (reward - a);
        }
        self.step_size *= STEP_SIZE_DECAY;
        self.num_current_states = 0;
    }

    fn reset(&mut self) {
        self.num_current_states = 0;
    }
}
