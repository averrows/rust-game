### How to write a function
To write a function it is good to write the behavior we expect. By listing every behavior that we expect, we ensure complex implementation doesn't run away from the expectation. take an example:
```
fn some_func(){
  - the camera should run
  - the point rendered relative to camera
  - position have to be updated after camera
}
```

### Entity Component System Architecture
#### **entity**
Entity is the equal representation of class in ECS. But map is not an entity, it is a resource. Resources are static, while entity are dynamic, its properties are changed overtime in response to user input.

No logic are associated with entity.

#### **component**
Component describe a property an entity may have. These components have description that represent the entity. One example is `Position` component on which describe where does the entity located. `Render` component describe how to render the entity. Etc.

No logic are associated with it either.

#### **system**
it query the entities and components and provide one element of game-play. One example that Herbert brought is `Render` system. It draws everything with the `Render` and `Position` component.

#### **resources**
shared data available to multiple system.

### Weapon
weapon need to live in another layer of screen.
it will be scaled to 2 or 3 times of the screen.
render the weapon in the position of the player transformed to the player screen + few units.
when attacking the weapon position need to move slightly to the position of the enemy.


### Attacking

There are some cases of attacking: [attacking_system]
- a weapon hit a target
- a player with certain potion use a weapon to hit the target:
here we need to increase or decrease the attack amount
- a weapon hit a target and the target have certain potion that reduce attack received
- a weapon hit a target and the target die. [killer_system] or here
- weapon has quotas
- weapon has owner
- weapon can attack from far away

#### **coarse design**
- retrieve WantsToAttack message
- get attack value of the weapon
- get the position of the target and the weapon
- the WantsToAttack message contains a tracker: frame_time
- if frame_time < duration: change the position of the weapon to go near to the target
- if frame_time past duration: change the position of the weapon back to the weapon owner
- update the health of the target
- if the health < 0, delete the health of the player


### level based system
- there will be 5 levels
- each level have different map and configuration of monsters