extern crate acquire;
extern crate brdgme_game;

use acquire::{Game, Phase, PubState};
use brdgme_game::command::Spec as CommandSpec;
use brdgme_game::bot::{Botter, BotCommand};

pub struct Brad;

impl Botter<Game> for Brad {
    fn commands(
        &mut self,
        player: usize,
        pub_state: &PubState,
        players: &[String],
        command_spec: &CommandSpec,
        game_id: Option<String>,
    ) -> Vec<BotCommand> {
        if pub_state.phase.whose_turn() != player {
            return vec![];
        }
        match pub_state.phase {
            Phase::Play(_) => handle_play_phase(player, pub_state),
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

fn handle_play_phase(player: usize, pub_state: &PubState) -> Vec<BotCommand> {
    // Consider each tile and give a quality score for each.
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
