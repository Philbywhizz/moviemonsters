use crate::prelude::*;

// A simple system that renders all the entities
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch Error");
}
