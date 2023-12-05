use crate::utils::{AppState, Args};
use druid::{Rect, Widget};
use clap::Parser;
use druid::RenderContext;
use druid::{Env, Color};
use druid::piet::{ImageFormat, InterpolationMode, Text, };
use druid::widget::prelude::*;

pub struct DrawingWidget;

impl Widget<AppState> for DrawingWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, _env: &Env) {
        // Handle user input events for drawing here
        match event {
            Event::MouseDown(e) => {
                data.is_drawing = true;
                println!("{}", data.is_drawing);
            }
            Event::MouseMove(e) => {
                if data.is_drawing {
                    data.drawing_points.move_to(e.pos);
                    println!("{:?}", data.drawing_points);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(e) => {
                data.is_drawing = false;
                println!("{}", data.is_drawing);
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
        let image = ctx.make_image(data.image.width() as usize, data.image.height() as usize, data.image.raw_pixels(), ImageFormat::RgbaSeparate).unwrap();
        ctx.draw_image(&image, Rect::new(0f64, 0f64, width, height), InterpolationMode::Bilinear);
        ctx.render_ctx.stroke(data.drawing_points.clone(), &Color::WHITE, 25.0);
    }
}