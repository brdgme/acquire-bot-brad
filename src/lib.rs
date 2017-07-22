extern crate acquire;
extern crate brdgme_game;

use acquire::{Game, Phase, PlayerState, CanEnd};
use acquire::board::{Loc, Tile};
use brdgme_game::command::Spec as CommandSpec;
use brdgme_game::bot::{Botter, BotCommand};

pub struct Brad;

impl Botter<Game> for Brad {
    fn commands(
        &mut self,
        player: usize,
        player_state: &PlayerState,
        players: &[String],
        command_spec: &CommandSpec,
        game_id: Option<String>,
    ) -> Vec<BotCommand> {
        if player_state.public.phase.whose_turn() != player {
            return vec![];
        }
        if player_state.public.can_end() == CanEnd::True {
            // Always end it if we can, as it's so rarely advantageous not to.
            return vec!["end".into()];
        }
        match player_state.public.phase {
            Phase::Play(_) => handle_play_phase(player_state),
            Phase::Buy { remaining, .. } => unimplemented!(),
            Phase::Found { at, .. } => unimplemented!(),
            Phase::ChooseMerger { at, .. } => unimplemented!(),
            Phase::SellOrTrade {
                corp,
                into,
                at,
                turn_player,
                ..
            } => unimplemented!(),
        }
    }
}

fn handle_play_phase(player_state: &PlayerState) -> Vec<BotCommand> {
    let available_corps = player_state.public.board.available_corps();
    // Consider each tile and give a quality score for each.
    let mut commands: Vec<BotCommand> = vec![];
    for t in &player_state.tiles {
        let neighbouring_corps = player_state.public.board.neighbouring_corps(t);
        if player_state.public.board.loc_neighbours_multiple_safe_corps(t) {
            // This tile is unplayable, ignore it.
            continue;
        }
        if t.neighbours().iter().find(|n| player_state.public.board.get_tile(*n) == Tile::Unincorporated).is_some() {
            // Founding location
            if available_corps.is_empty() {
                // Can't found anything at the moment.
                continue;
            }
            commands.push(BotCommand {
                quality: 200,
                commands: vec![format!("play {}", t)],
            });
        }
    }
    commands
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
