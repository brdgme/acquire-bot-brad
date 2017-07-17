extern crate acquire_bot_brad;
extern crate acquire;
extern crate brdgme_game;

use acquire_bot_brad::Brad;
use brdgme_game::bot::Fuzzer;

use std::io::stdout;

fn main() {
    let mut f = Fuzzer::new(Brad {});
    f.fuzz(&mut stdout());
}