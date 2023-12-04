use std::fs;
use std::process::{exit, id};
use druid::widget::{Align, Button, Painter, FillStrat, Flex, Image, Label, CrossAxisAlignment};
use druid::{AppLauncher, ImageBuf, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, WindowHandle, WindowLevel};
use druid::{Data, Lens};
use clap::Parser;
use druid::Value::{Size};
use image::{DynamicImage};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::Path;
use druid::RenderContext;
use druid::{Env, LensExt, Command, Color};
use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, FontDescriptor, Point, Rect, TextLayout,
};


#[derive(Debug, Clone, Data, Lens)]
struct AppState {
    // Define any application state data here
}

/// Annotation Tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

struct DrawingWidget;

impl Widget<AppState> for DrawingWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, _env: &Env) {
        // Handle user input events for drawing here

        match event {
            Event::MouseDown(e) => {
                println!("{:?}", e);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &AppState, _env: &Env) -> druid::Size {
        // Return the size of the drawing area
        druid::Size::new(1000.0, 400.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        // Implement your custom painting logic here
        // You can use ctx to draw on the widget
    }
}

fn ui_builder(path: String, image: DynamicImage) -> impl Widget<AppState> {
    // The label text will be computed dynamically based on the current locale and count
    let width = image.width();
    let height = image.height();
    let img = Image::new(ImageBuf::from_dynamic_image(image)).fix_size((width as f64)/3f64, (height as f64)/3f64);
    /*let pen = Button::new("Pen️").padding(5.0);
    let zoom_out = Button::new("Zoom In").padding(5.0);
    let zoom_in = Button::new("Zoom Out").padding(5.0);
    let fit = Button::new("Fit").padding(5.0);
    let save = Button::new("Save").padding(5.0);
    let delete = Button::new("Delete")
        .on_click(move |_, _, _| {
            let yes = MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_title(&format!("Are you sure you want to delete {}", path ))
                .set_text(&format!("Tools will be closed and this item will be moved to Bin."))
                .show_confirm()
                .unwrap();

            if yes {
               fs::remove_file(path.to_string()).unwrap();
                exit(0);
            }
        })
        .padding(5.0);*/

    // let first_row = Flex::row().with_child(pen).with_child(zoom_in).with_child(zoom_out).with_child(fit).padding(10.0);
    // let img_row = Flex::row().with_child(img).with_flex_child(DrawingWidget, 1.0).padding(10.0);
    // let second_row = Flex::row().with_child(save).with_child(delete).padding(10.0);
    // let container = Flex::column().with_child(first_row).with_child(img_row).with_child(second_row);
    // Align::centered(container)

    druid::widget::Flex::column()
        .with_child(img)
        .with_child(DrawingWidget)
}

fn main() -> Result<(), PlatformError> {
    let arg = Args::parse();
    if let Err(_) = fs::metadata(arg.path.to_string()) {
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title(&format!("File Not Found!"))
            .set_text(&format!("No such file \"{}\".\nPlease check that the file exists and try again.", Path::new(arg.path.to_string().as_str()).file_name().unwrap().to_str().unwrap()))
            .show_alert()
            .unwrap();
        exit(255);
    }
    let image = image::open(arg.path.to_string()).unwrap();
    let width = image.width();
    let height = image.height();

    let main_window = WindowDesc::new(ui_builder(arg.path.to_string(), image))
        .title(format!("Screen Crab Tools - {}", arg.path.to_string()))
        .window_size((width as f64, height as f64))
        .set_level(WindowLevel::AppWindow);

    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState{})
}