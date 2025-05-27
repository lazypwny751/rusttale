use tetra::Context;
use tetra::graphics::Rectangle;
use tetra::graphics::Color;
use tetra::graphics::DrawParams;
use tetra::graphics::mesh::Mesh;
use tetra::graphics::mesh::ShapeStyle;

pub fn draw_rect(ctx: &mut Context, rect: Rectangle, color: Color) -> tetra::Result {
    let mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, rect)?;
    mesh.draw(ctx, DrawParams::new().color(color));
    Ok(())
}

pub fn draw_line(ctx: &mut Context, x: f32, y: f32, width: f32, color: Color) -> tetra::Result {
    let line_rect = Rectangle::new(x, y, width, 1.0);
    draw_rect(ctx, line_rect, color)
}
