extern crate rand;
extern crate termion;

mod display;
mod entity;
mod frame;
mod game;
mod point;

use entity::{Apple, Direction, Snake};
use frame::Frame;
use game::Game;
use point::Point;

fn main() {
  let mut game = Game::new(
    Frame::new(30, 20),
    Snake::new(Point { x: 10, y: 10 }, Direction::DOWN),
    Apple { x: 5, y: 5 },
  );

  game.start();
}
