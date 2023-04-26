use crate::prelude::*;

#[system]
pub fn map_render(
  #[resource] map: &Map,
  #[resource] camera: &Camera
){
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(0);
  for y in camera.top_y..camera.bottom_y{
    for x in camera.left_x..camera.right_x {
      // point relative to global
      let point = Point::new(x,y);
      
      // offset
      // so that we can project the point in the camera perspective
      // and then render the screen as camera
      let offset = Point::new(camera.left_x, camera.top_y);
      
      // check if the global point is in the map bound
      if map.in_bounds(point) {
        let idx = map_idx(x, y);
        let glyph = match map.tiles[idx] {
          TileType::Floor => to_cp437('.'),
          TileType::Wall => to_cp437('#'),
          _ => to_cp437('#'),
        };
        // render relative to camera
        draw_batch.set(
          point - offset, 
          ColorPair::new(WHITE, BLACK)
          ,glyph);
      } else {
        draw_batch.set(
          point - offset, 
          ColorPair::new(WHITE, BLACK)
          ,to_cp437('#'));
      }
    }
  }
  draw_batch.submit(0).expect("submitting batch error");
}