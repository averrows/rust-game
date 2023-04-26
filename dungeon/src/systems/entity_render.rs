use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
  ecs: &SubWorld,
  #[resource] camera: &Camera
){
  let mut entities = <(&Point, &Render)>::query();
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(1);
  entities.iter(ecs).for_each(|(point,render)| {
    draw_batch.set(
      *point - camera.get_offset(), 
      render.color,
      render.glyph);
  });
  
  draw_batch.submit(5000).expect("Batch error");
}