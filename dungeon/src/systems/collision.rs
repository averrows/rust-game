use crate::{prelude::*};

#[system(for_each)]
#[read_component(Enemy)]
#[read_component(Player)]
#[write_component(Point)]
pub fn collision(
  entity: &Entity,
  want_to_move: &WantsToMove,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,  
){
  <(Entity, &Point)>::query()
    .filter(component::<Enemy>())
    .iter(ecs)
    .filter(|(_, pos)| **pos == want_to_move.destination)
    .for_each(|(target, _)| {
      commands.push(((), WantsToAttack{
        weapon: want_to_move.entity,
        target: *target
      }));
    })
}

