extern crate GolfManiaLib;

mod view;
mod login_logic;

use GolfManiaLib::model::{score_card};
use GolfManiaLib::model::map::*;
use view::console;

fn main() {
    let test : Map = MapBuilder::from_named(None).unwrap();

    for thing in test.into_iter() {
        //println!("{:?}", thing);
    }
}
