
use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToAttack)]
#[read_component(Health)]
#[read_component(Attack)]
#[write_component(Health)]
pub fn attacking(
  entity: &Entity,
  want_attack: &WantsToAttack,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer
){
  let attack_value = ecs
    .entry_ref(want_attack.weapon)
    .unwrap()
    .get_component::<Attack>()
    .map(|attack| attack.current)
    .unwrap_or(0);

  let target = ecs.entry_ref(want_attack.target).unwrap();
  
  match target.get_component::<Health>() {
    Ok(health) => {
      let mut new_health = Health {
        current: health.current,
        max: health.max
      };
      new_health.current -= attack_value;
      if new_health.current <= 0 {
        commands.remove(want_attack.target);
      } else {
        commands.add_component(want_attack.target, new_health);
      }
    },
    Err(_) => {}
  }

  commands.remove(*entity);
}