pub use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Player{}