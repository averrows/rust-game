mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
	pub use crate::map::*;
	pub use crate::map_builder::*;
	pub use crate::camera::*;
	pub use crate::components::*;
	pub use crate::spawner::*;
	pub use crate::systems::*;
	pub use crate::turn_state::*;

	pub use legion::*;
	pub use legion::systems::CommandBuffer;
	pub use legion::world::SubWorld;
	pub use bracket_lib::prelude::*;


	pub const SCREEN_WIDTH: i32 = 80;
	pub const SCREEN_HEIGHT: i32 = 50;
	pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
	pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}
use prelude::*;

fn main() {
	let ctx = BTermBuilder::new()
						.with_title("Fire Finder")
						.with_fps_cap(30.0)
						.with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
						.with_tile_dimensions(32, 32)
						.with_resource_path("resources/")
						.with_font("dungeonfont.png", 32, 32)
						.with_font("terminal8x8.png", 8, 8)
						.with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
						.with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
						.with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")
						.build().expect("fail build");
	main_loop(ctx, State::new());
}

// the state now consist of :
// - ecs that manage the entity and component
// - resources that manage the data that can be accessed by all entity and system
// - systems that manage the logic of the game
struct State {
	ecs: World,
	resources: Resources,
	enemy_system: Schedule,
	player_system: Schedule,
	waiting_input_system: Schedule,
}

impl GameState for State {
	fn tick(&mut self, ctx: &mut BTerm) {
		ctx.set_active_console(0);
		ctx.cls();
		ctx.set_active_console(1);
		ctx.cls();
		ctx.set_active_console(2);
		ctx.cls();

		self.resources.insert(ctx.key);
		ctx.set_active_console(0);
		self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
		
		let current_state = self.resources.get::<turn_state::TurnState>().unwrap().clone();
		match current_state {
			TurnState::WaitingInput => {
				self.waiting_input_system.execute(&mut self.ecs, &mut self.resources)
			} 
			TurnState::Player => {
				self.player_system.execute(&mut self.ecs, &mut self.resources)
			}
			TurnState::Enemy => {
				self.enemy_system.execute(&mut self.ecs, &mut self.resources)
			} 
		}
		
		// each system will have its own draw batch buffer
		// when the system is executing, it will stack the drawing command into the batch
		// after the system is done, it will draw the batch
		// this ensure that the drawing command is executed in the correct order
		// and preserve correctness of the drawing
		render_draw_buffer(ctx).expect("Render error");
	}
}

impl State {
	fn new() -> Self {
		let mut rng = RandomNumberGenerator::new();
		let map_builder = MapBuilder::new(&mut rng);

		let mut ecs = World::default();
		let mut resources = Resources::default();

		// insert map and camera to resources
		resources.insert(map_builder.map);
		resources.insert(Camera::new(map_builder.player_start));
		resources.insert(TurnState::WaitingInput);

		spawn_player(&mut ecs, map_builder.player_start);
		map_builder.rooms
			.iter()
			.skip(1)
			.map(|r| r.center())
			.for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
		
		State {
			ecs,
			resources,
			player_system: build_schedule_player(),
			enemy_system: build_schedule_enemy(),
			waiting_input_system: build_schedule_waiting_input(),
		}
	}
}




