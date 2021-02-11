use crate::display::CharacterDisplay;
use crate::point::Point;
use rand::Rng;
use std::io::{stdout, Bytes, Read, Write};
use termion::{
  async_stdin, clear, cursor,
  raw::{IntoRawMode, RawTerminal},
  AsyncReader,
};

type Grid = Vec<Vec<CharacterDisplay>>;

pub struct Frame {
  pub width: usize,
  pub height: usize,
  pub grid: Grid,
  pub stdout: RawTerminal<std::io::Stdout>,
  pub stdin: Bytes<AsyncReader>,
}

impl Frame {
  pub fn new(width: usize, height: usize) -> Frame {
    Frame {
      width,
      height,
      grid: vec![vec![CharacterDisplay::TILE; width]; height],
      stdout: stdout().into_raw_mode().unwrap(),
      stdin: async_stdin().bytes(),
    }
  }

  pub fn setup(&mut self) {
    write!(self.stdout, "{}", clear::All);
    self.stdout.flush().unwrap();
  }

  pub fn clear(&mut self) {
    for y in 0..self.grid.len() {
      for x in 0..self.grid[y].len() {
        self.grid[y][x] = CharacterDisplay::TILE
      }
    }
  }

  pub fn set_value(&mut self, point: Point, value: CharacterDisplay) {
    self.grid[point.y][point.x] = value;
  }

  pub fn get_random_location(&self) -> Point {
    let mut rng = rand::thread_rng();

    Point {
      x: rng.gen_range(0..self.width),
      y: rng.gen_range(0..self.height),
    }
  }

  pub fn print(&mut self) {
    write!(self.stdout, "{}", cursor::Goto(1, 1));
    write!(self.stdout, "{}", clear::AfterCursor);

    for row in &self.grid {
      write!(
        self.stdout,
        "{}\n\r",
        row
          .iter()
          .map(|col| format!("{}", col.as_char()))
          .collect::<String>()
      );
    }

    self.stdout.flush().unwrap();
  }
}
