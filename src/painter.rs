use crate::utils::{AppState, Action};
use druid::{Rect, Widget};
use druid::RenderContext;
use druid::{Env, Color};
use druid::kurbo::{Circle, Line};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::piet::Image;

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
                data.is_drawing = true;
                let mut action = Action::new(&data.selection);
                match action {
                    Action::Pencil(ref mut points) => { points.push(Circle::new(e.pos, 1.5)); }
                    Action::Pen(ref mut points) => { points.push(e.pos); }
                    Action::Highlighter(ref mut points) => { points.push(e.pos); }
                    Action::Eraser(ref mut points) => { /* TODO */ }
                    Action::Text(ref mut points) => { /* TODO */ }
                    _ => {}
                }
                data.actions.push(action);
                ctx.request_paint();
            }
            Event::MouseMove(e) => {
                if data.is_drawing {
                    if let Some(action) = data.actions.last_mut() {
                        match action {
                            Action::Pencil(points) => { points.push(Circle::new(e.pos, 1.5)); }
                            Action::Pen(points) => { points.push(e.pos); }
                            Action::Highlighter(points) => { points.push(e.pos); }
                            Action::Eraser(points) => { /* TODO */ }
                            Action::Text(points) => { /* TODO */ }
                            _ => {}
                        }
                    }
                    ctx.request_paint();
                }
            }
            Event::MouseUp(e) => {
                data.is_drawing = false;
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, data: &AppState, _env: &Env) -> druid::Size {
        // Return the size of the drawing area
        let width = ctx.window().get_size().width * 7.5f64/10f64;
        let result = druid::Size::new(width, (data.image.height() as f64 * width )/data.image.width() as f64);
        result
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        let width = ctx.size().width;
        let height = ctx.size().height;
        let image = ctx.make_image(data.image.width(), data.image.height(), data.image.raw_pixels(), ImageFormat::RgbaSeparate).unwrap();
        ctx.draw_image(&image, Rect::new(0f64, 0f64, width, height), InterpolationMode::Bilinear);
        for action in &data.actions {
            match action {
                Action::Pencil(action) => {
                    action.iter().for_each(|circle| ctx.fill(circle, &Color::BLACK));
                }
                Action::Highlighter(action) => {
                    if action.len() < 2 {
                        ctx.fill(Circle::new(*action.last().unwrap(), 5.0), &druid::Color::BLACK.with_alpha(0.25));
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.stroke(line, &druid::Color::BLACK.with_alpha(0.25), 10.0);
                        }
                    }
                }
                Action::Pen(action) => {
                    if action.len() < 2 {
                        ctx.fill(Circle::new(*action.last().unwrap(), 1.0), &druid::Color::BLACK);
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.stroke(line, &druid::Color::BLACK, 2.0);
                        }
                    }
                }
                Action::Eraser(action) => { /* TODO */ }
                Action::Text(action) => { /* TODO */ }
                _ => {}
            }
        }
    }
}