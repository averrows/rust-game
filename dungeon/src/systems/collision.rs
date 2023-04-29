use crate::{prelude::*, components};

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[write_component(Point)]
pub fn collision(
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,  
){
  // GOAL: remove monster when it is in the same position
  // idea: use command buffer
  let mut player_pos = Point::new(0, 0);
  <&Point>::query()
    .filter(component::<Player>())
    .iter_mut(ecs)
    .for_each(|pos| {
      player_pos.x = pos.x;
      player_pos.y = pos.y;
    });
  
  <(Entity, &Point)>::query()
    .filter(component::<Enemy>())
    .iter(ecs)
    .filter(|(_, pos)| **pos == player_pos)
    .for_each(|(entity, _)| {
      commands.remove(*entity);
    })
}