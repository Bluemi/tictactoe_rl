mod logic;
mod random;
mod winning;
mod agent;
mod game;

use crate::agent::random_agent::RandomAgent;
use crate::agent::interactive_agent::InteractiveAgent;
use crate::agent::tictactoe_agent::TicTacToeAgent;
use crate::game::Winner;

const NUM_TRAIN_GAMES: u32 = 1000 * 1000 * 10;
const NUM_TEST_GAMES: u32 = 1000;

fn main() {
	// ----------------- TRAIN -----------------
	let mut agent_x = TicTacToeAgent::new();
	let mut agent_o = TicTacToeAgent::new();
	// agent0.exploit();
	// agent1.exploit();

	let mut draw_counter = 0;
	let mut agent_x_counter = 0;
	let mut agent_o_counter = 0;

	for i in 0..NUM_TRAIN_GAMES {
		let winner = game::play(&mut agent_x, &mut agent_o, false);
		match winner {
			Winner::Draw => draw_counter += 1,
			Winner::AgentX => agent_x_counter += 1,
			Winner::AgentO => agent_o_counter += 1,
		}
		if i % 100000 == 0 {
			println!("epoch {}", i);
		}
	}

	println!("train agent X : {:4.1}% {:5}", (agent_x_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_x_counter);
	println!("train agent O : {:4.1}% {:5}", (agent_o_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_o_counter);
	println!("train draw    : {:4.1}% {:5}", (draw_counter as f32   / NUM_TEST_GAMES as f32) * 100.0, draw_counter);

	// ----------------- TEST -----------------
	agent_x.exploit();
	agent_o.exploit();

    let _interactive_agent = InteractiveAgent::new();
	let _random_agent = RandomAgent::new();

	draw_counter = 0;
	agent_x_counter = 0;
	agent_o_counter = 0;
	for _ in 0..NUM_TEST_GAMES {
		let winner = game::play(&mut agent_x, &mut agent_o, true);
        match winner {
			Winner::Draw => draw_counter += 1,
			Winner::AgentX => agent_x_counter += 1,
			Winner::AgentO => agent_o_counter += 1,
		}
	}

 	println!("agent X : {:4.1}% {:5}", (agent_x_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_x_counter);
	println!("agent O : {:4.1}% {:5}", (agent_o_counter as f32 / NUM_TEST_GAMES as f32) * 100.0, agent_o_counter);
	println!("draw    : {:4.1}% {:5}", (draw_counter as f32   / NUM_TEST_GAMES as f32) * 100.0, draw_counter);
}

