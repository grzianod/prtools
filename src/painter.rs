use std::thread;
use std::time::Duration;

use crate::utils::{AppState, Action, Transformation};
use druid::{Cursor, Rect, Widget, WidgetExt, WindowDesc, AppLauncher, WindowConfig, Code, TextLayout, ImageBuf, Affine, FontDescriptor, FontFamily};
use druid::RenderContext;
use druid::{Env, Color};
use druid::{Data, Lens};
use druid::kurbo::{Circle, Line, Point, Vec2, Ellipse};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::Event;
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use num_traits::cast::FromPrimitive;
use druid::piet::{Text, TextLayoutBuilder};
use screenshots::Screen;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetSystemMetrics, SM_CYCAPTION};

/*use cocoa::appkit::{
    CGFloat, NSApp, NSApplication, NSAutoresizingMaskOptions, NSBackingStoreBuffered, NSColor,
    NSEvent, NSView, NSViewHeightSizable, NSViewWidthSizable, NSWindow, NSWindowStyleMask,
};
use objc::declare::ClassDecl;
use objc::rc::WeakPtr;
use objc::runtime::{Class, Object, Protocol, Sel};
use objc::{class, msg_send, sel, sel_impl};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use core_graphics::context::CGContext;
use core_graphics::display::CGSize;
use core_graphics::image::CGImage;
use core_graphics::sys::CGContextRef;
use druid_shell::piet;
use druid::piet::CoreGraphicsContext;*/

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


// use druid::piet::d2d::Bitmap;
/* 
fn convert_bitmap_to_dynamic_image(bitmap: Bitmap) -> DynamicImage {
}

// Implement the save function
fn save_image(bitmap: Bitmap, path: &str) -> Result<(), image::ImageError> {
    let dynamic_image = convert_bitmap_to_dynamic_image(bitmap);
    dynamic_image.save(path)
}
*/

pub struct DrawingWidget;

impl Widget<AppState> for DrawingWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, _env: &Env) {
        // Handle user input events for drawing here
        match event {
            Event::KeyDown(key) => {
                println!("here");
                if data.is_writing_text {
                    if key.code.eq(&Code::Enter) {
                        data.is_writing_text = false;
                        return;
                    }
                    if let Some(action) = data.actions.last_mut() {
                        if let Action::Text(affine, _, string, _) = action {
                            *affine = data.affine;
                            string.push(key.code.to_string().chars().next().unwrap());
                        }
                    }
                }
            }
            Event::Paste(clipboard) => {
                if data.is_writing_text {
                    if let Some(action) = data.actions.last_mut() {
                        if let Action::Text(affine, _, string, _) = action {
                            *affine = data.affine;
                            *string = clipboard.get_string().unwrap();
                        }
                    }
                }
            }
            Event::Zoom(value) => {
                data.zoom *= (1f64 + value);
                println!("{}", data.zoom);
            }
            Event::MouseDown(e) => {
                if data.is_picking_color {
                    ctx.set_cursor(&Cursor::Pointer);
                    return;
                }
                data.is_drawing = true;
                let mut action = Action::new(&data.selection);
                ctx.set_cursor(&Cursor::Crosshair);
                match action {
                    Action::Pen(ref mut affine, ref mut points, ref mut color, ref mut stroke) => {
                        points.push(e.pos);
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Highlighter(ref mut affine, ref mut points, ref mut color, ref mut stroke) => {
                        points.push(e.pos);
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Rectangle(ref mut affine, ref mut start_point, ref mut end_point, ref mut color, ref mut fill, ref mut stroke) => {
                        *start_point = e.pos;
                        *end_point = e.pos;
                        *fill = data.fill_color;
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Circle(ref mut affine, ref mut center, ref mut radius, ref mut color, ref mut fill, ref mut stroke) => {
                        *center = e.pos;
                        *fill = data.fill_color;
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Ellipse(ref mut affine, ref mut start_point, ref mut end_point, ref mut color, ref mut fill, ref mut stroke) => {
                        *start_point = e.pos;
                        *end_point = e.pos;
                        *fill = data.fill_color;
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Arrow(ref mut affine, ref mut start_point, ref mut end_point, ref mut color, ref mut stroke) => {
                        *start_point = e.pos;
                        *end_point = e.pos;
                        *color = data.color;
                        *stroke = data.stroke;
                        *affine = data.affine;
                    }
                    Action::Text(ref mut affine, ref mut position, ref mut text, ref mut color) => {
                        if data.is_writing_text { return; }
                        *position = e.pos;
                        *color = data.color;
                        *affine = data.affine;
                        // Set a flag or state indicating that text input is needed
                        data.is_writing_text = true;
                    }
                }
                data.actions.push(action);
                ctx.request_paint();
            }
            Event::MouseMove(e) => {
                if data.is_picking_color {
                    ctx.set_cursor(&Cursor::Pointer);
                    return;
                }
                ctx.set_cursor(&Cursor::Crosshair);
                if data.is_drawing {
                    if let Some(action) = data.actions.last_mut() {
                        match action {
                            Action::Pen(_, points, _, _) => { points.push(e.pos); }
                            Action::Highlighter(_, points, color, _) => { points.push(e.pos); }
                            Action::Rectangle(_, _, end_point, _, _, _) => {
                                *end_point = e.pos;
                            }
                            Action::Circle(_, center, radius, _, _, _) => {
                                *radius = f64::sqrt(num_traits::pow((center.x - e.pos.x), 2) + num_traits::pow((center.y - e.pos.y), 2));
                            }
                            Action::Ellipse(_, _, end_point, _, _, _) => {
                                *end_point = e.pos;
                            }
                            Action::Arrow(_, _, end_point, _, _) => {
                                *end_point = e.pos;
                            }
                            _ => {}
                        }
                    }
                    ctx.request_paint();
                }
            }
            Event::MouseUp(e) => {
                if data.is_picking_color {
                    let img = image::open(data.image_path.to_string()).unwrap();
                    let x = (img.width() * u32::from_f64(e.pos.x).unwrap()) / u32::from_f64(ctx.size().width).unwrap();
                    let y = (img.height() * u32::from_f64(e.pos.y).unwrap()) / u32::from_f64(ctx.size().height).unwrap();
                    let pixel = img.get_pixel(x, y);
                    data.color = Color::rgba8(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
                    ctx.set_cursor(&Cursor::Arrow);
                    data.custom_color = true;
                    data.is_picking_color = false;
                    return;
                }
                if let Some(Action::Rectangle(_, _, end_point, _, _, _)) = data.actions.last_mut() {
                    *end_point = e.pos;
                }
                if let Some(Action::Circle(_, center, radius, _, _, _)) = data.actions.last_mut() {
                    *radius = f64::sqrt(num_traits::pow((center.x - e.pos.x), 2) + num_traits::pow((center.y - e.pos.y), 2));
                }
                if let Some(Action::Ellipse(_, _, end_point, _, _, _)) = data.actions.last_mut() {
                    *end_point = e.pos;
                }
                if let Some(Action::Arrow(_, _, _, _, _)) = data.actions.last_mut() {}
                if let Some(Action::Text(_, position, text, color)) = data.actions.last_mut() {
                    if data.is_writing_text { return; }
                    *position = e.pos;
                    *color = data.color;
                    // Set a flag or state indicating that text input is needed
                    data.is_writing_text = true;
                }
                data.is_drawing = false;
                data.update.set(true);
                ctx.set_cursor(&Cursor::Arrow);
                data.repaint = true;
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

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, _env: &Env) -> druid::Size {
        // Return the size of the drawing area
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = druid::Size::new(ctx.window().get_size().width, ctx.window().get_size().height - 28.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        let width = ctx.size().width;
        let height = ctx.size().height;

        ctx.with_save(|ctx| {
            ctx.render_ctx.transform(data.affine);
            if data.affine == Affine::FLIP_Y {
                ctx.transform(Affine::translate((0.0, -height)));
            }
            if data.affine == Affine::FLIP_X {
                ctx.transform(Affine::translate((-width, 0.0)));
            }

            let image = ctx.render_ctx.make_image(data.image.width(), data.image.height(), data.image.raw_pixels(), ImageFormat::RgbaSeparate).unwrap();
            ctx.render_ctx.draw_image(&image, Rect::new(0f64, 0f64, width, height), InterpolationMode::Bilinear);
        });

        let width = ctx.size().width;
        let height = ctx.size().height;

        /*let result = draw_with_cgcontext(ctx);
        let w = result.as_ref().width();
        println!("{}", w);*/

        for action in &data.actions {
            match action {
                Action::Highlighter(affine, action, color, stroke) => {
                    if action.len() < 2 {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), stroke * 2f64), &color.with_alpha(0.25));
                        });

                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            ctx.with_save(|ctx| {
                                let current_affine = data.affine * *affine;
                                ctx.render_ctx.transform(current_affine);
                                if current_affine == Affine::FLIP_Y {
                                    ctx.transform(Affine::translate((0.0, -height)));
                                }
                                if current_affine == Affine::FLIP_X {
                                    ctx.transform(Affine::translate((-width, 0.0)));
                                }
                                if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                    ctx.transform(Affine::translate((-width, -height)));
                                }
                                let line = Line::new(*start, *end);
                                ctx.render_ctx.stroke(line, &color.with_alpha(0.25), stroke * 3f64)
                            });
                        }
                    }
                }
                Action::Pen(affine, action, color, stroke) => {
                    if action.len() < 2 {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.fill(Circle::new(*action.last().unwrap(), stroke / 2f64), color);
                        });
                    }
                    for pair in action.windows(2) {
                        if let [start, end] = pair {
                            ctx.with_save(|ctx| {
                                let current_affine = data.affine * *affine;
                                ctx.render_ctx.transform(current_affine);
                                if current_affine == Affine::FLIP_Y {
                                    ctx.transform(Affine::translate((0.0, -height)));
                                }
                                if current_affine == Affine::FLIP_X {
                                    ctx.transform(Affine::translate((-width, 0.0)));
                                }
                                if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                    ctx.transform(Affine::translate((-width, -height)));
                                }
                                let line = Line::new(*start, *end);
                                ctx.render_ctx.stroke(line, color, *stroke);
                            });
                        }
                    }
                }
                Action::Rectangle(affine, start_point, end_point, color, fill, stroke) => {
                    if *fill {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.fill_even_odd(Rect::new(start_point.x, start_point.y, end_point.x, end_point.y), color);
                        });
                        } else {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.stroke(Rect::new(start_point.x, start_point.y, end_point.x, end_point.y), color, *stroke);
                        });
                        }
                }
                Action::Circle(affine, center, radius, color, fill, stroke) => {
                    if *fill {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.fill_even_odd(Circle::new(*center, *radius), &data.color);
                        });
                        } else {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.stroke(Circle::new(*center, *radius), &data.color, *stroke);
                        });
                        }
                }
                Action::Ellipse(affine, start_point, end_point, color, fill, stroke) => {
                    if *fill {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.fill_even_odd(Ellipse::from_rect(Rect::new(start_point.x, start_point.y, end_point.x, end_point.y)), color);
                        });
                        } else {
                        ctx.with_save(|ctx| {
                            let current_affine = data.affine * *affine;
                            ctx.render_ctx.transform(current_affine);
                            if current_affine == Affine::FLIP_Y {
                                ctx.transform(Affine::translate((0.0, -height)));
                            }
                            if current_affine == Affine::FLIP_X {
                                ctx.transform(Affine::translate((-width, 0.0)));
                            }
                            if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                                ctx.transform(Affine::translate((-width, -height)));
                            }
                            ctx.render_ctx.stroke(Ellipse::from_rect(Rect::new(start_point.x, start_point.y, end_point.x, end_point.y)), color, *stroke);
                        });
                        }
                }
                Action::Arrow(affine, start_point, end_point, color, stroke) => {
                    ctx.with_save(|ctx| {
                        let current_affine = data.affine * *affine;
                        ctx.render_ctx.transform(current_affine);
                        if current_affine == Affine::FLIP_Y {
                            ctx.transform(Affine::translate((0.0, -height)));
                        }
                        if current_affine == Affine::FLIP_X {
                            ctx.transform(Affine::translate((-width, 0.0)));
                        }
                        if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                            ctx.transform(Affine::translate((-width, -height)));
                        }
                        // Draw the line
                        let line = Line::new(*start_point, *end_point);
                        let len = line.length();
                        ctx.render_ctx.stroke(line, color, *stroke);
                        // Calculate the arrowhead points
                        let arrowhead_length = len / 10f64;
                        let arrowhead_width = len * 5f64 / 100f64;
                        let (left_point, right_point) = calculate_arrowhead(*start_point, *end_point, arrowhead_length, arrowhead_width);
                        // Draw the arrowhead
                        let arrowhead = Line::new(left_point, *end_point);
                        ctx.render_ctx.stroke(arrowhead, color, *stroke);
                        let arrowhead = Line::new(right_point, *end_point);
                        ctx.render_ctx.stroke(arrowhead, color, *stroke);
                    });
                }
                Action::Text(affine, pos, text, color) => {
                    ctx.with_save(|ctx| {
                        let current_affine = data.affine * *affine;
                        ctx.render_ctx.transform(current_affine);
                        if current_affine == Affine::FLIP_Y {
                            ctx.transform(Affine::translate((0.0, -height)));
                        }
                        if current_affine == Affine::FLIP_X {
                            ctx.transform(Affine::translate((-width, 0.0)));
                        }
                        if current_affine == Affine::FLIP_X * Affine::FLIP_Y {
                            ctx.transform(Affine::translate((-width, -height)));
                        }
                        let mut layout = TextLayout::<String>::from_text(text.to_string());
                        layout.set_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(24.0));
                        layout.set_text_color(*color);
                        layout.rebuild_if_needed(ctx.text(), env);
                        layout.draw(ctx, *pos);
                    });
                }
            }
        }

        if data.save {
            
            let dx = ctx.to_window(Point::new(0f64, 0f64)).x as u32;
            let dy = ctx.to_window(Point::new(0f64, 0f64)).y as u32;
            println!("{} {}", dx, dy);
            let screens = Screen::all().unwrap();
            let screen = screens.first().unwrap();
            let x = ctx.window().get_position().x as i32;
            let y = ctx.window().get_position().y as i32;
            let width = ctx.window().get_size().width as u32;
            let height = ctx.window().get_size().height as u32;
            thread::sleep(Duration::from_millis(300));
            #[cfg(target_os = "windows")]
            let title_bar_height = unsafe { GetSystemMetrics(SM_CYCAPTION) } as u32;
            let image = screen.capture_area(x + dx as i32, y + dy as i32, width, height).unwrap();
            image.save(data.image_path.as_str()).unwrap();
        }
    }
}

/*#[cfg(target_os = "macos")]
fn draw_with_cgcontext(paint_ctx: &mut druid::PaintCtx) -> CGContext {
    use core_graphics::context::{CGContextRef, CGContext};
    use std::os::raw::c_void;

    // Access the raw CGContextRef
    unsafe {
        let raw_context = paint_ctx.render_ctx;

        // Perform your drawing operations using Core Graphics here
       CGContext::from_existing_context_ptr(raw_context as *mut core_graphics::sys::CGContext)

        // Make sure to release the CGContextRef when done
        // CGContext::release(cg_context); // Usually not necessary, as Rust's ownership system will handle it
    }
}*/