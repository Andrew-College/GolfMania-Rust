extern crate GolfManiaLib as glib;

mod view;
mod login_logic;

use glib::model::{score_card};
use glib::model::map::*;
use view::console;

fn main() {
    let test : Map = MapBuilder::from_named(None).unwrap();

    for thing in test.into_iter() {
        //println!("{:?}", thing);
    }
}
