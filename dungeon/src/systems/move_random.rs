use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn move_random(
  ecs: &mut SubWorld,
  #[resource] map: &Map 
){
  // Goal: every monster walk randomly
  // Idea: randomly add 1 or -1 to x or y
  let mut rng = RandomNumberGenerator::new();
  <&mut Point>::query()
    .filter(component::<MovingRandomly>())
    .iter_mut(ecs)
    .for_each(|pos| {
      let diff = match rng.range(0, 5){
        0 => Point::new(1, 0),
        1 => Point::new(-1, 0),
        2 => Point::new(0, -1),
        3 => Point::new(0, 1),
        _ => Point::new(0, 0)
      };
      let destination = *pos + diff;
      if map.can_enter_tile(destination){
        *pos = destination;
      }
    })
}