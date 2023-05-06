mod player_input;
mod map_render;
mod entity_render;
mod collision;
mod move_random;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod attacking;
use crate::prelude::*;

use self::{collision::collision_system, attacking::attacking_system};


pub fn build_schedule_waiting_input() -> Schedule {
  Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(tooltips::tooltips_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
    .flush()
    .build()
}

pub fn build_schedule_player() -> Schedule {
  Schedule::builder()
    .add_system(collision_system())
    .flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(attacking_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_schedule_enemy() -> Schedule {
  Schedule::builder()
    .add_system(move_random::move_random_system())
    .flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
    .add_system(end_turn::end_turn_system())
    .build()
}