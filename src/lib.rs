extern crate acquire;
extern crate brdgme_game;

use acquire::{CanEnd, Game, Phase, PlayerState, PubPlayer};
use acquire::corp::Corp;
use acquire::board::{Loc, Tile};
use brdgme_game::command::Spec as CommandSpec;
use brdgme_game::bot::{BotCommand, Botter};

use std::collections::HashSet;
use std::cmp::max;

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

fn bonuses(pub_players: &[PubPlayer], corp: &Corp) -> (HashSet<usize>, HashSet<usize>) {
    let mut major: usize = 0;
    let mut major_players: HashSet<usize> = HashSet::new();
    let mut minor: usize = 0;
    let mut minor_players: HashSet<usize> = HashSet::new();
    for (p, p_state) in pub_players.iter().enumerate() {
        let p_shares = p_state.shares.get(corp).cloned().unwrap_or(0);
        if p_shares == 0 {
            continue;
        }
        if p_shares > major {
            minor = major;
            minor_players = major_players;
            major = p_shares;
            major_players = HashSet::new();
        }
        if p_shares == major {
            major_players.insert(p);
        } else {
            if p_shares > minor {
                minor = p_shares;
                minor_players = HashSet::new();
            }
            if p_shares == minor {
                minor_players.insert(p);
            }
        }
    }
    (major_players, minor_players)
}

fn handle_play_phase(player_state: &PlayerState) -> Vec<BotCommand> {
    let available_corps = player_state.public.board.available_corps();
    // Consider each tile and give a quality score for each.
    let mut commands: Vec<BotCommand> = vec![];
    for t in &player_state.tiles {
        let neighbouring_corps = player_state.public.board.neighbouring_corps(t);
        if player_state
            .public
            .board
            .loc_neighbours_multiple_safe_corps(t)
        {
            // This tile is unplayable, ignore it.
            continue;
        }
        if neighbouring_corps.len() > 1 {
            // Merge location
            let (from, into) = player_state.public.board.merge_candidates(t);
            // Quality starts quite low as some merges are bad for us (if we have no shares)
            let mut quality: u8 = 0;
            for f in &from {
                let (major, minor) = bonuses(&player_state.public.players, f);
                // If we have the major bonus for any of the from corps, it's a good move
                if major.contains(&player_state.player) {
                    quality = max(200, quality);
                } else if minor.contains(&player_state.player) {
                    quality = max(150, quality);
                } else {
                    let p_shares = player_state.public.players[player_state.player]
                        .shares
                        .get(f)
                        .cloned()
                        .unwrap_or(0);
                    quality = max(p_shares as u8 * 9, quality);
                }
            }
            commands.push(BotCommand {
                quality,
                commands: vec![format!("play {}", t)],
            });
        } else if neighbouring_corps.len() == 1 {
            // Growing a corporation, prefer to do it to other players' corps.
            let corp = neighbouring_corps.iter().next().unwrap();
            let (major, minor) = bonuses(&player_state.public.players, corp);
            let quality: u8 = if major.contains(&player_state.player) {
                50
            } else if minor.contains(&player_state.player) {
                100
            } else {
                150
            };
            commands.push(BotCommand {
                quality,
                commands: vec![format!("play {}", t)],
            });
        } else if t.neighbours()
            .iter()
            .find(|n| {
                player_state.public.board.get_tile(*n) == Tile::Unincorporated
            })
            .is_some()
        {
            // Founding location
            if available_corps.is_empty() {
                // Can't found anything at the moment.
                continue;
            }
            commands.push(BotCommand {
                quality: 200,
                commands: vec![format!("play {}", t)],
            });
        } else {
            // Just a random tile somewhere
            commands.push(BotCommand {
                quality: 75,
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
