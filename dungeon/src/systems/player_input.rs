use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] turn_state: &mut TurnState){
  
  if let Some(key) = key {
    let delta = match key {
      VirtualKeyCode::W => Point::new(0, -1),
      VirtualKeyCode::A => Point::new(-1, 0),
      VirtualKeyCode::S => Point::new(0, 1),
      VirtualKeyCode::D => Point::new(1, 0),
      _ => Point::new(0, 0)
    };

    if delta.x != 0 || delta.y != 0 {
      let mut players = <(Entity,&Point)>::query()
        .filter(component::<Player>());
      players.iter_mut(ecs).for_each(|(entity,pos)| {
        let destination = *pos + delta;
        commands.push(((), WantsToMove{entity: *entity, destination}));        
    })
  }
  *turn_state = TurnState::Player; 
}
}
