use crate::{prelude::*};

#[system(for_each)]
#[read_component(Enemy)]
#[read_component(Player)]
#[write_component(Point)]
pub fn collision(
  entity: &Entity,
  player: &Player,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,  
){


  match ecs.entry_ref(*entity)
  .unwrap().get_component::<Point>() {
    Ok(point) => {
    <(Entity, &Point)>::query()
      .filter(component::<Enemy>())
      .iter(ecs)
      .filter(|(_, pos)| **pos == *point)
      .for_each(|(target, _)| {
        commands.push(((), WantsToAttack{
          weapon: *entity,
          target: *target
        }));
      })
    },
    Err(_) => {}
  }

}