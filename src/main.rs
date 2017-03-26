extern crate GolfManiaLib as glib;

mod view;
mod login_logic;

use glib::model::{score_card, map};
use glib::model::map::*;
use view::console;

fn main() {
    let new_score = MapBuilder::from_named(None);
    
    println!("{:?}", new_score.unwrap().board().first().unwrap().first());
}
