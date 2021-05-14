use crate::prelude::*;

// A simple system that renders all the entities
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &mut Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1); // draw on entity layer
    let camera_offset = Point::new(camera.left_x, camera.top_y);
    let viewport_offset = Point::new(1, 1);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            if camera.in_view(pos) {
                draw_batch.set(
                    *pos - camera_offset + viewport_offset,
                    render.color,
                    render.glyph,
                );
            }
        });

    draw_batch.submit(5000).expect("Batch Error");
}
