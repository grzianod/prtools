mod utils;
mod painter;

use std::fs;
use std::process::{exit};
use druid::widget::{Align, Button, Flex};
use druid::{AppLauncher, ImageBuf, PlatformError, Screen, Widget, WidgetExt, WindowDesc, WindowLevel};
use druid::{Data, Lens};
use clap::Parser;
use clap::Args;
use druid::RenderContext;
use druid::{LensExt};
use druid::piet::{Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use crate::utils::AppState;
use crate::painter::DrawingWidget;

fn ui_builder() -> impl Widget<AppState> {
    // The label text will be computed dynamically based on the current locale and count

    let pen = Button::new("Pen️").padding(5.0);
    let zoom_out = Button::new("Zoom In").padding(5.0);
    let zoom_in = Button::new("Zoom Out").padding(5.0);
    let fit = Button::new("Fit").padding(5.0);
    let save = Button::new("Save").padding(5.0).on_click(|ctx, data, env| {
        println!("SAVE");
    });
    let delete = Button::new("Delete")
        .on_click(move |ctx, data: &mut AppState, env| {
            if utils::dialog_delete_file(data.image_path.to_string()) {
                fs::remove_file(data.image_path.to_string()).unwrap();
                exit(0);
            }
        })
        .padding(5.0);

    let first_row = Flex::row().with_child(pen).with_child(zoom_in).with_child(zoom_out).with_child(fit).padding(5.0);
    let drawing_row = Flex::row().with_child(DrawingWidget).padding(5.0);
    let second_row = Flex::row().with_child(save).with_child(delete).padding(5.0);
    let container = Flex::column().with_child(first_row).with_child(drawing_row).with_child(second_row);
    Align::centered(container)
}

fn main() -> Result<(), PlatformError> {
    let arg = utils::Args::parse();
    //check if the file exists
    if let Err(_) = fs::metadata(arg.path.to_string()) {
        utils::dialog_file_not_found(arg.path.to_string());
        exit(255);
    }

    let monitor = Screen::get_monitors().first().unwrap().virtual_work_rect();

    let main_window = WindowDesc::new(ui_builder())
        .title(format!("Screen Crab Tools - {}", arg.path.to_string()))
        .window_size((monitor.width()*8f64/10f64, monitor.height()*9f64/10f64))
        .set_level(WindowLevel::AppWindow)
        .resizable(false);

    let image = image::open(arg.path.to_string()).unwrap();
    let image_width = image.width();
    let image_height = image.height();
    let imagebuf = ImageBuf::from_dynamic_image(image);

    let initial_state = AppState::new(imagebuf, arg.path.to_string(), image_width as f64, image_height as f64);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(|env, _| {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, druid::Color::WHITE);
        })
        .launch(initial_state)
}