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