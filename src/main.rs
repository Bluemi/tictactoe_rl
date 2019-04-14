mod game;
mod random;
mod winning;
mod ai;

use crate::game::{State, EntryKind, StatePosition, is_winning, print_state,
                  get_interactive_state_position, get_random_state_position, toggle_entry_kind};
use crate::random::RandomGenerator;


enum AgentMoveKind {
	Interactive,
	Random,
}


struct AgentMove {
	agent_move_kind: AgentMoveKind,
	random_generator: RandomGenerator,
}


impl AgentMove {
	fn new(agent_move_kind: AgentMoveKind) -> AgentMove {
		AgentMove {
			agent_move_kind,
			random_generator: RandomGenerator::new()
		}
	}

	fn next_move(&mut self, state: State, entry_kind: EntryKind) -> StatePosition {
		match self.agent_move_kind {
			AgentMoveKind::Interactive => get_interactive_state_position(state, entry_kind),
			AgentMoveKind::Random => get_random_state_position(state,
															   entry_kind,
															   &mut self.random_generator),
		}
	}

	fn toggle(&mut self) {
		match self.agent_move_kind {
			AgentMoveKind::Interactive => self.agent_move_kind = AgentMoveKind::Random,
			AgentMoveKind::Random => self.agent_move_kind = AgentMoveKind::Interactive,
		};
	}
}


fn main() {
	let mut state: State = 0;
	let mut entry_kind: EntryKind = EntryKind::X;
	let mut agent_move: AgentMove = AgentMove::new(AgentMoveKind::Interactive);

	loop {
		let new_state_position = agent_move.next_move(state, entry_kind);

		if new_state_position == 0 { break; }

		state |= new_state_position;
		print_state(state);

		if is_winning(state, entry_kind) {
			println!("{} has won!", entry_kind.get_text());
			break;
		}

		agent_move.toggle();
		entry_kind = toggle_entry_kind(entry_kind);
	}
}
