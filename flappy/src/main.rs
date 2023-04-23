use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() {
  println!("Hello, world!");
  if SCREEN_HEIGHT == 50 && SCREEN_WIDTH == 80 {
    let context = BTermBuilder::simple80x50()
      .with_title("Flappy Things")
      .build().expect("Failed to build BTerm");
    main_loop(context, State::new());
  }
}

struct State {
  player: Player,
  frame_time: f32,
  mode: GameMode,
  obstacles: Vec<Obstacle>,
  score: i32
}
impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm){
    self.run_mode(ctx);
  }
}
impl State {
  fn new() -> Self {
    State {
      player: Player::new(0,10),
      frame_time: 0.0,
      mode: GameMode::Menu,
      obstacles: Vec::new(),
      score: 0
    }
  }

  fn main_menu(&mut self, ctx: &mut BTerm){
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Things");
    ctx.print_centered(8, "(P) Play game");
    ctx.print_centered(11, "(Q) Quit game");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn restart(&mut self) {
    self.player = Player::new(0,0);

    self.obstacles.push(Obstacle::new(SCREEN_WIDTH, self.score));

    self.mode = GameMode::Playing;
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls_bg(NAVY);
    self.frame_time += ctx.frame_time_ms;

    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;
      self.player.gravity_and_move();
    }

    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.flap()
    }

    self.player.render(ctx);
    for mut obstacle in self.obstacles.iter_mut() {
      obstacle.render(ctx, self.player.x);
    }
    let mut is_hit_obstacle = false;
    for obstacle in self.obstacles.iter() {
      is_hit_obstacle = is_hit_obstacle || obstacle.is_hit_obstacle(&self.player);
      if is_hit_obstacle {return;}
    }

    ctx.print(0,0, "Print SPACE to flap.");
    if self.player.y > SCREEN_HEIGHT || is_hit_obstacle{
      self.mode = GameMode::End;
    }
  }

  fn dead(&mut self, ctx: &mut BTerm) {
    self.main_menu(ctx);
  }

  fn run_mode(&mut self, ctx: &mut BTerm) {
    match self.mode {
      GameMode::Menu => self.main_menu(ctx),
      GameMode::Playing => self.play(ctx),
      GameMode::End => self.dead(ctx),
    }
  }
}

enum GameMode {
  Menu,
  Playing,
  End
}

struct Player {
  x: i32,
  y: i32,
  velocity: f32
}

impl Player {
  fn new (x: i32, y: i32) -> Self {
    Player {
      x,
      y,
      velocity: 0.0
    }
  }

  fn render(&mut self, ctx: &mut BTerm) {
    ctx.set(
      self.x,
      self.y,
      YELLOW,
      BLACK,
      to_cp437('@')
    )
  }

  fn gravity_and_move(&mut self){
    if self.velocity < 2.0 {
      self.velocity += 0.2;
    }

    self.y += self.velocity as i32;

    self.x += 1;
    if self.y < 0 {
      self.y = 0;
      self.velocity += 3.0;
    }
  }

  fn flap(&mut self){
    self.velocity -= 2.0;
  }
}

struct Obstacle {
  x: i32,
  gap_y: i32,
  size: i32
}

impl Obstacle {
  fn new(x: i32, score: i32) -> Self {
    let mut random = RandomNumberGenerator::new();
    Obstacle {
      x,
      gap_y: random.range(10, 40),
      size: i32::max(2, 20 - score)
    }
  }

  fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
    let screen_x = self.x - player_x;
    let half_size = self.size / 2;

    for y in 0..self.gap_y - half_size {
      ctx.set(
        screen_x,
        y,
        RED,
        BLACK,
        to_cp437('|')
      )
    }

    for y in self.gap_y + half_size..SCREEN_HEIGHT {
      ctx.set(
        screen_x,
        y,
        RED,
        BLACK,
        to_cp437('|')
      )
    }
  }

  fn is_hit_obstacle(&self, player: &Player) -> bool {
    let half_size = self.size / 2;
    let screen_x = self.x - player.x;
    let does_x_equals = player.x == screen_x;
    let does_y_equals = player.y <= self.gap_y - half_size || player.y >= self.gap_y + half_size;
    return does_x_equals && does_y_equals;
  }
}

