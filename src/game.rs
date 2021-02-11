use crate::entity::{Apple, Snake};
use crate::frame::Frame;
use crate::point::Point;
use std::thread::sleep;
use std::time::Duration;

pub struct Game {
  frame: Frame,
  snake: Snake,
  apple: Point,
  finished: bool,
}

impl Game {
  pub fn new(frame: Frame, snake: Snake, apple: Apple) -> Game {
    Game {
      frame,
      snake,
      apple,
      finished: false,
    }
  }

  pub fn start(&mut self) {
    self.frame.setup();

    loop {
      self.update();

      if self.finished {
        break;
      }

      self.draw();

      sleep(Duration::from_millis(1000 / 5));
    }
  }

  pub fn draw(&mut self) {
    self.frame.clear();

    self.apple.draw(&mut self.frame);
    self.snake.draw(&mut self.frame);

    self.frame.print();
  }

  pub fn update(&mut self) {
    let key = self.frame.stdin.next().unwrap_or(Ok(0)).unwrap();

    if key == b'q' {
      self.finished = true;
      return;
    }

    self.snake.update(key, &self.frame, &mut self.apple);

    if self.snake.hit_self() {
      self.finished = true;
      return;
    }
  }
}
