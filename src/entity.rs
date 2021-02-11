use crate::display::CharacterDisplay;
use crate::frame::Frame;
use crate::point::Point;
use std::collections::VecDeque;
use std::ptr;

pub type Apple = Point;

pub struct Snake {
  segments: VecDeque<Point>,
  direction: Direction,
}

#[derive(PartialEq)]
pub enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

impl Apple {
  pub fn draw(&self, frame: &mut Frame) {
    frame.set_value(*self, CharacterDisplay::APPLE);
  }

  pub fn set_location(&mut self, point: Point) {
    self.x = point.x;
    self.y = point.y;
  }
}

impl Snake {
  pub fn new(start: Point, direction: Direction) -> Snake {
    let mut segments = VecDeque::new();
    segments.push_back(start);

    Snake {
      segments,
      direction,
    }
  }

  pub fn hit_self(&self) -> bool {
    let head = self.segments.back().unwrap();

    for point in &self.segments {
      if point.x == head.x && point.y == head.y && !ptr::eq(point, head) {
        return true;
      }
    }

    false
  }

  pub fn update(&mut self, key: u8, frame: &Frame, apple: &mut Apple) {
    let current_head = self.segments.back().unwrap();

    match key {
      b'w' => {
        if self.direction != Direction::DOWN {
          self.direction = Direction::UP
        }
      }
      b's' => {
        if self.direction != Direction::UP {
          self.direction = Direction::DOWN
        }
      }
      b'a' => {
        if self.direction != Direction::RIGHT {
          self.direction = Direction::LEFT
        }
      }
      b'd' => {
        if self.direction != Direction::LEFT {
          self.direction = Direction::RIGHT
        }
      }
      _ => {}
    };

    let head = match self.direction {
      Direction::UP => Point {
        x: current_head.x,
        y: ((current_head.y as isize) - 1).rem_euclid(frame.height as isize) as usize,
      },
      Direction::DOWN => Point {
        x: current_head.x,
        y: (current_head.y + 1) % frame.height,
      },
      Direction::LEFT => Point {
        x: ((current_head.x as isize) - 1).rem_euclid(frame.width as isize) as usize,
        y: current_head.y,
      },
      Direction::RIGHT => Point {
        x: (current_head.x + 1) % frame.width,
        y: current_head.y,
      },
    };

    if head.x != apple.x || head.y != apple.y {
      self.segments.pop_front();
    } else {
      apple.set_location(frame.get_random_location());
    }

    self.segments.push_back(head);
  }

  pub fn draw(&self, frame: &mut Frame) {
    for point in &self.segments {
      frame.set_value(*point, CharacterDisplay::SNAKE)
    }
  }
}
