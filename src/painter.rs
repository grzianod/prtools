use crate::utils::{AppState, Action};
use druid::{Cursor, Rect, Widget, WidgetExt, WindowDesc, AppLauncher, WindowConfig};
use druid::RenderContext;
use druid::{Env, Color};
use druid::{Data, Lens};
use druid::kurbo::{Circle, Line, Point, Vec2, Ellipse};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::Event;
use druid::widget::{TextBox, Flex, Button};
use image::GenericImageView;
use num_traits::cast::FromPrimitive;
use druid::piet::{Text, TextLayoutBuilder};

fn calculate_arrowhead(start: Point, end: Point, arrowhead_length: f64, arrowhead_width: f64) -> (Point, Point) {
    let direction = (end - start).normalize();
    let perpendicular = Vec2::new(-direction.y, direction.x) * arrowhead_width / 2.0;
    let arrowhead_base = end - direction * arrowhead_length;
    let left_point = arrowhead_base + perpendicular;
    let right_point = arrowhead_base - perpendicular;
    (left_point, right_point)
}

#[derive(Clone, Data, Lens)]
struct TextInputState {
    text: String,
}

fn text_input_window() -> WindowDesc<TextInputState> {
    let layout = Flex::column()
        .with_child(TextBox::new().lens(TextInputState::text))
        .with_child(Button::new("Submit").on_click(|ctx, data: &mut TextInputState, _env| {
            // Handle text submission here
            // For example, you might want to update some shared state
            // or close the text input window
        }));

    WindowDesc::new(layout)
}

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
                    Action::Pen(ref mut points, ref color, ref mut stroke) => { points.push(e.pos); color.set(data.color); stroke.set(data.stroke) }
                    Action::Highlighter(ref mut points, ref color, ref stroke) => { points.push(e.pos); color.set(data.color); stroke.set(data.stroke) }
                    Action::Rectangle(ref mut start_point, ref mut end_point, ref color, ref mut fill, ref mut stroke) => {
                        start_point.set(e.pos);
                        end_point.set(e.pos);
                        fill.set(data.fill_color);
                        color.set(data.color);
                        stroke.set(data.stroke);
                        // The second point (x1, y1) will be set on MouseUp
                    }
                    Action::Circle(ref mut center, ref mut radius, ref color, ref fill, ref mut stroke ) => {
                        center.set(e.pos);
                        fill.set(data.fill_color);
                        color.set(data.color);
                        stroke.set(data.stroke);
                    }
                    Action::Ellipse(ref mut points, ref color, ref fill, ref stroke) => {
                        points.push(e.pos);
                        fill.set(data.fill_color);
                        color.set(data.color);
                        stroke.set(data.stroke);
                    }
                    Action::Arrow(ref mut start_point, ref mut end_point, ref color, ref stroke) => {
                        start_point.set(e.pos);
                        end_point.set(e.pos);
                        color.set(data.color);
                        stroke.set(data.stroke);
                    }
                    Action::Text(ref mut position, ref mut text, ref color) => {
                        *position = e.pos;
                        // Set a flag or state indicating that text input is needed
                        data.text_ready = true;
                    }
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
                            Action::Pen(points, color, _) => { points.push(e.pos); }
                            Action::Highlighter(points, color, _) => { points.push(e.pos); }
                            Action::Rectangle(_, end_point, _, _, _) => {
                                end_point.set(e.pos);
                            }
                            Action::Circle(center, radius, _, _, _) => {
                                radius.set( f64::sqrt(num_traits::pow((center.get().x-e.pos.x),2) + num_traits::pow((center.get().y-e.pos.y), 2)));
                            }
                            Action::Ellipse(_, _, _, _) => {
                            }
                            Action::Arrow(_, end_point, _, _) => {
                                end_point.set(e.pos);
                            }
                            Action::Text(position, ref mut text, ref color) => {

                            }
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
                if let Some(Action::Rectangle(_, end_point, _, _, _)) = data.actions.last_mut() {
                    end_point.set(e.pos);
                }
                if let Some(Action::Circle(center, radius, _, _, _)) = data.actions.last_mut() {
                    radius.set( f64::sqrt(num_traits::pow((center.get().x-e.pos.x),2) + num_traits::pow((center.get().y-e.pos.y), 2)));
                }
                if let Some(Action::Ellipse(points, _, _, _)) = data.actions.last_mut() {
                    points.push(e.pos);
                }
                if let Some(Action::Arrow(_, _, _, _)) = data.actions.last_mut() {

                }
                if let Some(Action::Text(position, text, _)) = data.actions.last_mut() {
                    if data.text_ready {
                        // Finalize the placement of the text
                        // The actual text string is already updated from the dialog callback
                        // Reset the flag
                        data.text_ready = false;
                    }
                }
                data.is_drawing = false;
                ctx.set_cursor(&Cursor::Arrow);
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &AppState, data: &AppState, _env: &Env) {
        if data.repaint {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, data: &AppState, _env: &Env) -> druid::Size {
        // Return the size of the drawing area
        druid::Size::new(ctx.window().get_size().width, ctx.window().get_size().height - 28.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        let width = ctx.size().width;
        let height = ctx.size().height;
        let image = ctx.render_ctx.make_image(data.image.width(), data.image.height(), data.image.raw_pixels(), ImageFormat::RgbaSeparate).unwrap();
        ctx.render_ctx.draw_image(&image, Rect::new(0f64, 0f64, width, height), InterpolationMode::Bilinear);
        for action in &data.actions {
            match action {
                Action::Highlighter(action, color, stroke) => {
                    if action.len() < 2 {
                        ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), stroke.get()), &color.get().with_alpha(0.25));
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.render_ctx.stroke(line, &color.get().with_alpha(0.25), stroke.get()*3f64);
                        }
                    }
                }
                Action::Pen(action, color, stroke) => {
                    if action.len() < 2 {
                        ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), stroke.get()/2f64), &color.get());
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            let line = Line::new(*start, *end);
                            ctx.render_ctx.stroke(line, &color.get(), stroke.get());
                        }
                    }
                }
                Action::Rectangle(start_point, end_point, color, fill, stroke) => {
                    if fill.get() {
                        ctx.render_ctx.fill_even_odd(Rect::new(start_point.get().x, start_point.get().y, end_point.get().x, end_point.get().y), &color.get());
                    }
                    else {
                        ctx.render_ctx.stroke(Rect::new(start_point.get().x, start_point.get().y, end_point.get().x, end_point.get().y), &color.get(), stroke.get());
                    }
                    }
                Action::Circle(center, radius, color, fill, stroke) => {
                    if fill.get() {
                        ctx.render_ctx.fill_even_odd(Circle::new(center.get(), radius.get()), &data.color);
                    }
                    else {
                        ctx.render_ctx.stroke(Circle::new(center.get(), radius.get()), &data.color, stroke.get());
                    }
                    }
                Action::Ellipse(points, color, fill, stroke) => {
                    let points = points.clone();
                    let start = points.first().unwrap();
                    let end = points.last().unwrap();
                    let ellipse_center = Point::new((start.x + end.x)/2.0, (start.y + end.y)/2.0);
                    let ellipse = Ellipse::new(ellipse_center, Vec2::new((start.x - end.x).abs()/2.0, (start.y - end.y).abs()/2.0), 0.0);
                    ctx.render_ctx.stroke(ellipse, &color.get(), stroke.get());
                }
                Action::Arrow(start_point, end_point, color, stroke) => {
                        // Draw the line
                        let line = Line::new(start_point.get(), end_point.get());
                        ctx.render_ctx.stroke(line, &color.get(), stroke.get());
                        // Calculate the arrowhead points
                        let arrowhead_length = stroke.get()*3f64;
                        let arrowhead_width = stroke.get()*3f64;
                        let (left_point, right_point) = calculate_arrowhead(start_point.get(), end_point.get(), arrowhead_length, arrowhead_width);
                        // Draw the arrowhead
                        let arrowhead = Line::new(left_point, end_point.get());
                        ctx.render_ctx.stroke(arrowhead, &color.get(), stroke.get());
                        let arrowhead = Line::new(right_point, end_point.get());
                        ctx.render_ctx.stroke(arrowhead, &color.get(), stroke.get());

                }
                Action::Text(pos, text, _) => {
                    let layout = ctx.text().new_text_layout(text.to_string()).build().unwrap();
                    ctx.render_ctx.draw_text(&layout, *pos);
                }
            }
        }
    }
}