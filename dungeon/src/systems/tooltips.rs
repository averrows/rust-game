
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Player)]
pub fn tooltips(
  ecs: &SubWorld,
  #[resource] mouse_pos: &Point,
  #[resource] camera: &Camera
){
  let mut positions = <(Entity, &Point, &Name)>::query();

  let offset = camera.get_offset();
  let map_pos = *mouse_pos + offset;
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(2);
  positions
    .iter(ecs)
    .filter(|(_,pos,_)| **pos == map_pos)
    .for_each(|(entity, _, name)| {
      let screen_pos = *mouse_pos * 4;
      draw_batch.print(screen_pos, display_health_wording(ecs, entity, name));
    });
  
  let mut players = <(&Player, &Point)>::query();
  
  players
    .iter(ecs)
    .for_each(|(_, player_pos)| {
      positions.iter(ecs)
        .filter(|(_,pos,_)| {
          let distance = f32::sqrt((player_pos.x - pos.x).pow(2) as f32 + (player_pos.y - pos.y).pow(2) as f32);
          distance < 5.0
        })
        .for_each(|(entity, pos, name)| {
          let screen_pos = (pos.clone() - offset) * 4;
          draw_batch.print(screen_pos, display_health_wording(ecs, entity, name));
    })});
  draw_batch.submit(10100).expect("Batch error");
}

fn display_health_wording(ecs: &SubWorld, entity: &Entity, name: &Name) -> String {
  if let Ok(health) = ecs.entry_ref(*entity)
    .unwrap()
    .get_component::<Health>()
  {
     return format!("{} [{} / {}]", &name.0, health.current, health.max);
  } else {
     return name.0.clone();
  }
}