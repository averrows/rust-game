pub use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Player{}
pub struct Enemy{}

pub struct MovingRandomly{}
pub struct WantsToMove {
  pub entity: Entity,
  pub destination: Point
}
pub struct Health {
  pub current: i32,
  pub max: i32
}

pub struct Name(pub String);