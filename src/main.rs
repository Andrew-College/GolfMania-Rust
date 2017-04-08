#![feature(start)]
extern crate GolfManiaLib as glib;

mod view;
mod login_logic;

use glib::model::{score_card};
use glib::model::map::*;
use view::console;

#[start]
fn main( argc : isize, argv: *const *const u8 ) -> isize {
    let test : Map = MapBuilder::from_named(None).unwrap();

    for thing in test.into_iter() {
        //println!("{:?}", thing);
    }

    1337
}
