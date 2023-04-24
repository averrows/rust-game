mod map;
mod player;
mod map_builder;
mod camera;
mod prelude {
	pub use bracket_lib::prelude::*;
	pub use crate::map::*;
	pub use crate::player::*;
	pub use crate::map_builder::*;
	pub use crate::camera::*;
	pub use legion::*;
	pub use legion::systems::CommandBuffer;
	pub use legion::world::SubWorld;

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
						.with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
						.with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
						.build().expect("fail build");
	main_loop(ctx, State::new());
}

struct State {
	map: Map,
	player: Player,
	camera: Camera
}
impl GameState for State {
	fn tick(&mut self, ctx: &mut BTerm) {
		ctx.set_active_console(0);
		ctx.cls();
		ctx.set_active_console(1);
		ctx.cls();

		self.player.update(ctx, &self.map, &mut self.camera);
		self.map.render(ctx, &self.camera);
		self.player.render(ctx, &self.camera);
	}
}

impl State {
	fn new() -> Self {
		let mut rng = RandomNumberGenerator::new();
		let map_builder = MapBuilder::new(&mut rng);
		State {
			map: map_builder.map,
			player: Player::new(map_builder.player_start),
			camera: Camera::new(map_builder.player_start)
		}
	}
}




