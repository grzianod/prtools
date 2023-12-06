use crate::utils::{AppState, Action};
use druid::{Cursor, ImageBuf, Rect, Widget};
use druid::RenderContext;
use druid::{Env, Color};
use druid::kurbo::{Circle, Line};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::piet::Image;
use druid::Event;
use image::GenericImageView;
use num_traits::cast::FromPrimitive;


pub struct DrawingWidget;

impl Widget<AppState> for DrawingWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, _env: &Env) {
        // Handle user input events for drawing here
        match event {
            Event::Paste(clipboard) => {
                println!("Pasted!");
            }
            Event::Zoom(value) => {
                println!("Zoomed! {}", value);
            }
            Event::MouseDown(e) => {
                if data.is_picking_color { ctx.set_cursor(&Cursor::Pointer); return; }
                data.is_drawing = true;
                let mut action = Action::new(&data.selection);
                ctx.set_cursor(&Cursor::Crosshair);
                match action {
                    Action::Pen(ref mut points, ref color) => { points.push(e.pos); color.set(data.color); }
                    Action::Highlighter(ref mut points, ref color) => { points.push(e.pos); color.set(data.color); }
                    Action::Text(ref mut points, ref color) => { /* TODO */ }
                }
                data.actions.push(action);
                ctx.request_paint();
            }
            Event::MouseMove(e) => {
                if data.is_picking_color { ctx.set_cursor(&Cursor::Pointer); return; }
                ctx.set_cursor(&Cursor::Crosshair);
                if data.is_drawing {
                    if let Some(action) = data.actions.last_mut() {
                        match action {
                            Action::Pen(points, color) => { points.push(e.pos); }
                            Action::Highlighter(points, color) => { points.push(e.pos); }
                            Action::Text(points, color) => { /* TODO */ }
                            _ => {}
                        }
                    }
                    ctx.request_paint();
                }
            }
            Event::MouseUp(e) => {
                if data.is_picking_color {
                        let img = image::open(data.image_path.to_string()).unwrap();
                        let x = (img.width() * u32::from_f64(e.pos.x).unwrap())/u32::from_f64(ctx.size().width).unwrap();
                        let y = (img.height() * u32::from_f64(e.pos.y).unwrap())/u32::from_f64(ctx.size().height).unwrap();
                        let pixel = img.get_pixel(x, y);
                        data.color = Color::rgba8(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
                        ctx.set_cursor(&Cursor::Arrow);
                        data.custom_color = true;
                        data.is_picking_color = false;
                        return;

                }
                data.is_drawing = false;
                ctx.set_cursor(&Cursor::Arrow);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        if _data.repaint {
            _ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, data: &AppState, _env: &Env) -> druid::Size {
        // Return the size of the drawing area
        let width = ctx.window().get_size().width * 8f64/10f64;
        let result = druid::Size::new(width, (data.image.height() as f64 * width )/data.image.width() as f64);
        result
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        let width = ctx.size().width;
        let height = ctx.size().height;
        let image = ctx.render_ctx.make_image(data.image.width(), data.image.height(), data.image.raw_pixels(), ImageFormat::RgbaSeparate).unwrap();
        ctx.render_ctx.draw_image(&image, Rect::new(0f64, 0f64, width, height), InterpolationMode::Bilinear);
        for action in &data.actions {
            match action {
                Action::Highlighter(action, color) => {
                    if action.len() < 2 {
                        ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), 5.0), &color.get().with_alpha(0.25));
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.render_ctx.stroke(line, &color.get().with_alpha(0.25), 10.0);
                        }
                    }
                }
                Action::Pen(action, color) => {
                    if action.len() < 2 {
                        ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), 1.0), &color.get());
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.render_ctx.stroke(line, &color.get(), 2.0);
                        }
                    }
                }
                Action::Text(action, color) => { /* TODO */ }
                _ => {}
            }
        }
    }
}