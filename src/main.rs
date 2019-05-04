mod logic;
mod random;
mod winning;
mod agent;
mod game;

use crate::agent::random_agent::RandomAgent;
use crate::agent::interactive_agent::InteractiveAgent;
use crate::agent::tictactoe_agent::TicTacToeAgent;
use crate::game::Winner;
use crate::logic::EntryKind;

const NUM_TRAIN_GAMES: u32 = 1000 * 1000 * 10;
const NUM_TEST_GAMES: u32 = 1000;

fn main() {
	// ----------------- TRAIN -----------------
	let mut agent_x = TicTacToeAgent::new(EntryKind::X);
	let mut agent_o = TicTacToeAgent::new(EntryKind::O);
    // agent_x.exploit();
	// agent_o.exploit();

	let mut draw_counter = 0;
	let mut agent_x_counter = 0;
	let mut agent_o_counter = 0;

	for _ in 0..NUM_TRAIN_GAMES {
		let winner = game::play(&mut agent_x, &mut agent_o, false);
		match winner {
			Winner::Draw => draw_counter += 1,
			Winner::AgentX => agent_x_counter += 1,
			Winner::AgentO => agent_o_counter += 1,
		}
	}

	println!("train agent X : {:4.1}% {:5}", (agent_x_counter as f32 / NUM_TRAIN_GAMES as f32) * 100.0, agent_x_counter);
	println!("train agent O : {:4.1}% {:5}", (agent_o_counter as f32 / NUM_TRAIN_GAMES as f32) * 100.0, agent_o_counter);
	println!("train draw    : {:4.1}% {:5}", (draw_counter as f32   / NUM_TRAIN_GAMES as f32) * 100.0, draw_counter);

	// ----------------- TEST -----------------
	agent_x.exploit();
	agent_o.exploit();
	// agent_x.debug();
	// agent_o.debug();

    let mut _interactive_agent = InteractiveAgent::new();
	let mut _random_agent = RandomAgent::new();

	draw_counter = 0;
	agent_x_counter = 0;
	agent_o_counter = 0;
	for _ in 0..NUM_TEST_GAMES {
		let winner = game::play(&mut agent_x, &mut _random_agent, true);
        match winner {
			Winner::Draw => draw_counter += 1,
			Winner::AgentX => agent_x_counter += 1,
			Winner::AgentO => agent_o_counter += 1,
		}
	}

	println!("\n --------------- results ---------------\n");
 	println!("agent X : {:4.1}% {:5}", (agent_x_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_x_counter);
	println!("agent O : {:4.1}% {:5}", (agent_o_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_o_counter);
	println!("draw    : {:4.1}% {:5}", (draw_counter as f32   / NUM_TEST_GAMES as f32) * 100.0, draw_counter);
}
