extern crate GolfManiaLib;

use GolfManiaLib::Model::{score_card, map};

fn main() {
    let new_score = map::Map::from_named(None);

    println!("{:?}", new_score.board().first().unwrap().first());
}
