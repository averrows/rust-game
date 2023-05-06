use crate::prelude::*;
pub struct Camera {
  pub left_x: i32,
  pub right_x: i32,
  pub top_y: i32,
  pub bottom_y: i32,
}

impl Camera {
  pub fn new(player_position: Point ) -> Self {
    Camera {
      left_x: player_position.x - DISPLAY_WIDTH / 2,
      right_x: player_position.x + DISPLAY_WIDTH / 2,
      top_y: player_position.y - DISPLAY_HEIGHT / 2,
      bottom_y: player_position.y + DISPLAY_HEIGHT / 2,
    }
  }

  pub fn on_player_move(&mut self, player_position: Point) {
    if player_position.x == self.left_x - 1 {
      self.left_x -= DISPLAY_WIDTH;
      self.right_x -= DISPLAY_WIDTH;
    }
    if player_position.x == self.right_x + 1 {
      self.left_x += DISPLAY_WIDTH;
      self.right_x += DISPLAY_WIDTH;
    }
    if player_position.y == self.top_y - 1 {
      self.top_y -= DISPLAY_HEIGHT;
      self.bottom_y -= DISPLAY_HEIGHT;
    }
    if player_position.y == self.bottom_y + 1 {
      self.top_y += DISPLAY_HEIGHT;
      self.bottom_y += DISPLAY_HEIGHT;
    }
  }

  pub fn get_offset(&self) -> Point {
    Point::new(self.left_x, self.top_y)
  }
}