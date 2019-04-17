use std::io::Write;

use crate::random::RandomGenerator;
use crate::winning::WINNER_PATTERNS;

pub type State = u32;
pub type StatePosition = u32;
pub type IndexPosition = u32;

const X_BITMASK: State = 0b000000000111111111;
const O_BITMASK: State = 0b111111111000000000;

#[derive(Copy, Clone)]
pub enum EntryKind {
	X,
	O
}

impl EntryKind {
	pub fn get_shift_size(&self) -> u32 {
		return match self {
			EntryKind::X => 0,
			EntryKind::O => 9,
		}
	}

	pub fn get_text(&self) -> String {
		return match self {
			EntryKind::X => String::from("X"),
			EntryKind::O => String::from("O"),
		}
	}
}

/*
 * IndexPosition EntryKind StatePosition
 *      0            X       00...001
 *      1            X       00...010
 *      2            X       00...100
 *      0            O       00...001 000000000
 *      1            O       00...010 000000000
 */
fn index_position_to_state_position(index_position: IndexPosition) -> StatePosition {
	return 1 << index_position;
}

/*
pub fn print_state_position(state_position: StatePosition) {
	let mut result: String = String::with_capacity(18);
	
	for i in 0..18 {
		let bit_mask = 1 << (17-i);
		if (bit_mask & state_position) != 0 {
			result.push('1');
		} else {
			result.push('0');
		}
	}
	println!("{} {}", result, state_position);
}
*/

pub fn print_state(state: State) {
	let mut result: String = String::with_capacity(11);
	let x_state: State = X_BITMASK & state;
	let o_state: State = (O_BITMASK & state) >> 9;

	for i in 0..9 {
		let bit_mask = 1 << i;
		if (bit_mask & x_state) != 0 {
			result.push('X');
		} else if (bit_mask & o_state) != 0 {
			result.push('O');
		} else {
			result.push('~');
		}
	}
	result.insert(3, '\n');
	result.insert(7, '\n');
	println!("{}\n", result);
}

fn get_empty_fields(state: State) -> State {
	return !((state & X_BITMASK) | (state >> 9));
}

pub fn get_possible_state_positions(possible_state_positions: &mut [u32; 9], state: State) -> u32 {
	let empty_fields: State = get_empty_fields(state);

	let mut index = 0;

	for i in 0..9 {
		let bit_mask = 1 << (8-i);
		if (bit_mask & empty_fields) != 0 {
			possible_state_positions[index] = bit_mask;
			index += 1;
		}
	}
	return index as u32;
}

pub fn get_random_state_position(state: State, random_generator: &mut RandomGenerator) -> StatePosition {
	let mut possible_state_positions: [u32; 9] = [0; 9];
	let num_possible_state_positions = get_possible_state_positions(&mut possible_state_positions, state);

	if num_possible_state_positions == 0 { return 0; }

	let index = random_generator.next().unwrap() % num_possible_state_positions;

	return possible_state_positions[index as usize];
}

fn get_input() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed");
	buffer
}

fn get_input_int() -> u32 {
	return get_input().trim().parse::<u32>().unwrap();
}

pub fn get_interactive_state_position(state: State) -> StatePosition {
	let empty_fields: State = get_empty_fields(state);

	if (empty_fields & X_BITMASK) == 0 { return 0; }

	let mut state_position: StatePosition;

	loop {
		print!("enter position: ");
		std::io::stdout().flush().unwrap();
		let index_position = get_input_int();

		if index_position > 8 {
			println!("position should be between 0-8");
			state_position = 0;
		} else {
			state_position = index_position_to_state_position(index_position);
		}

		if (state_position & empty_fields) != 0 {
			break;
		} else {
			println!("position {} is already used", index_position);
		}
	}

	return state_position;
}

pub fn is_winning(state: State, entry_kind: EntryKind) -> bool {
	let kind_state: State = (state >> entry_kind.get_shift_size()) & X_BITMASK;
	for &winner_pattern in &WINNER_PATTERNS[..] {
		if (winner_pattern & kind_state) == winner_pattern {
			return true;
		}
	}
	return false;
}
