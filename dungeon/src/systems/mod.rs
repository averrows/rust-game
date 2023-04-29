mod player_input;
mod map_render;
mod entity_render;
mod collision;
mod move_random;

use crate::prelude::*;


pub fn build_schedule() -> Schedule {
  Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(collision::collision_system())
    // .add_system(move_random::move_random_system())
    .build()
}