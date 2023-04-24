use crate::prelude::*;
const NUM_ROOMS: usize = 30;

pub struct MapBuilder {
  pub map: Map,
  pub rooms: Vec<Rect>,
  pub player_start: Point,
}

impl MapBuilder {
  pub fn new(rng: &mut RandomNumberGenerator) -> Self {
    let mut mb = MapBuilder {
      map: Map::new(),
      rooms: Vec::new(),
      player_start: Point::zero(),
    };
    mb.fill(TileType::Wall);
    mb.build_random_rooms(rng);
    mb.build_corridors(rng);
    mb.player_start = mb.rooms[0].center();
    mb
  }
  
  fn fill(&mut self, tile: TileType) {
    self.map.tiles.iter_mut().for_each(|t| *t = tile);
  }
  fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
    // build non intersecting rooms
    while self.rooms.len() < NUM_ROOMS {
      let room: Rect = Rect::with_size(
        rng.range(1, SCREEN_WIDTH - 10), 
        rng.range(1, SCREEN_HEIGHT - 10),
        rng.range(2,10), 
        rng.range(2,10));
      let mut overlap = false;
      for other_room in self.rooms.iter() {
        if room.intersect(other_room) {
          overlap = true;
        }

        if overlap {
          break;
        }
      }

      if !overlap {
        // assign tiles to floor
        room.for_each(|p| {
          let idx = map_idx(p.x, p.y);
          self.map.tiles[idx] = TileType::Floor;
        });
        self.rooms.push(room);
      }
    }
  }

  pub fn apply_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
    for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
      if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
        self.map.tiles[idx] = TileType::Floor;
      }
    }
  }
  pub fn apply_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
    for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
      if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
        self.map.tiles[idx] = TileType::Floor;
      }
    }
  }
  pub fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
    let mut rooms = self.rooms.clone();
    rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

    for (i, room) in rooms.iter().enumerate().skip(1) {
      let prev = rooms[i-1].center();
      let new = room.center();

      if rng.range(0, 2) == 1 {
        self.apply_horizontal_corridor(prev.x, new.x, prev.y);
        self.apply_vertical_corridor(prev.y, new.y, new.x);
      } else {
        self.apply_vertical_corridor(prev.y, new.y, prev.x);
        self.apply_horizontal_corridor(prev.x, new.x, new.y);
      }
    }
  }
}