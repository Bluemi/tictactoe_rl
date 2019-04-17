use crate::agent::{Agent, GameMove};
use crate::logic::{State, StatePosition, print_state, is_winning, EntryKind};

const WIN_REWARD: f64 = 0.0;
const DRAW_REWARD: f64 = 1.0;
const LOSE_REWARD: f64 = -1.0;

#[derive(Clone, Copy)]
pub enum Winner {
    Draw,
    AgentX,
    AgentO,
}

impl Winner {
    fn from_agent_number(agent_number: u8) -> Winner {
        if agent_number == 0 {
            return Winner::AgentX;
        }
        Winner::AgentO
    }
}

pub fn play<AgentTypeX: Agent,
            AgentTypeO: Agent>(
    agent_x: &mut AgentTypeX,
    agent_o: &mut AgentTypeO,
    show: bool) -> Winner
{
    let mut state: State = 0;
    let mut agent_number = 0;

    let mut winner = Winner::Draw;
    loop {
        let entry_kind: EntryKind = get_entry_kind(agent_number);

        let game_move: GameMove = get_next_move(agent_x, agent_o, agent_number, state);
        if game_move == 0 {
            if show {
                println!("draw!");
            }
            break;
        }

        apply_game_move(game_move, entry_kind, &mut state);

        if show {
            print_state(state);
        }

        if is_winning(state, entry_kind) {
            if show {
                println!("{} has won!", entry_kind.get_text());
            }
            winner = Winner::from_agent_number(agent_number);
            break;
        }

        agent_number = 1 - agent_number;
    }
    apply_reward(agent_x, agent_o, winner);

    agent_o.reset();
    agent_x.reset();

    return winner;
}

fn apply_game_move(game_move: GameMove, entry_kind: EntryKind, state: &mut State) {
    let state_position: StatePosition = match entry_kind {
        EntryKind::X => game_move,
        EntryKind::O => game_move << 9,
    };
    *state |= state_position;
}

fn get_entry_kind(agent_number: u8) -> EntryKind {
    if agent_number == 0 {
        EntryKind::X
    } else {
        EntryKind::O
    }
}

fn get_next_move<AgentTypeX: Agent, AgentTypeO: Agent>(
    agent_x: &mut AgentTypeX,
    agent_o: &mut AgentTypeO,
    agent_number: u8,
    state: State) -> StatePosition
{
    let game_move: StatePosition = if agent_number == 0 {
        agent_x.next_move(state)
    } else {
        agent_o.next_move(state)
    };
    return game_move;
}

fn apply_reward<AgentTypeX: Agent, AgentTypeO: Agent>(
    agent_x: &mut AgentTypeX,
    agent_o: &mut AgentTypeO,
    winner: Winner)
{
    match winner {
        Winner::Draw => {
            agent_x.apply_reward(DRAW_REWARD);
            agent_o.apply_reward(DRAW_REWARD);
        },
        Winner::AgentX => {
            agent_x.apply_reward(WIN_REWARD);
            agent_o.apply_reward(LOSE_REWARD);
        },
        Winner::AgentO => {
            agent_x.apply_reward(LOSE_REWARD);
            agent_o.apply_reward(WIN_REWARD);
        },
    }
}
