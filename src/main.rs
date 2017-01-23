extern crate GolfManiaLib;

mod login_logic;

use GolfManiaLib::Model::{score_card, map};

use login_logic::login;

fn main() {
    let new_score = map::Map::from_named(None);

    println!("{:?}", new_score.board().first().unwrap().first());
}
