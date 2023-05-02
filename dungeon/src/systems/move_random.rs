use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn move_random(
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] map: &Map 
){
  // Goal: every monster walk randomly
  // Idea: randomly add 1 or -1 to x or y
  let mut rng = RandomNumberGenerator::new();
  <(Entity,&Point)>::query()
    .filter(component::<MovingRandomly>())
    .iter_mut(ecs)
    .for_each(|(entity,pos)| {
      let diff = match rng.range(0, 20){
        0 => Point::new(1, 0),
        1 => Point::new(-1, 0),
        2 => Point::new(0, -1),
        3 => Point::new(0, 1),
        _ => Point::new(0, 0)
      };
      let destination = *pos + diff;
      commands.push(((), WantsToMove{
        entity: *entity,
        destination
      }));
    })
}