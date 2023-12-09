#![windows_subsystem = "windows"]
mod utils;
mod painter;
mod menu;

use std::fs;
use std::process::{exit};
use druid::widget::{Align, Flex, Scroll};
use druid::{AppLauncher, Color, PlatformError, Screen, Widget, WidgetExt, WindowDesc};
use clap::Parser;
use crate::utils::{AppState};
use crate::painter::DrawingWidget;


fn ui_builder() -> impl Widget<AppState> {
    let drawing = Flex::row().with_child(DrawingWidget).padding(0.0);
    Align::centered(Scroll::new(drawing))
}

fn main() -> Result<(), PlatformError> {
    let arg = utils::Args::parse();
    //check if the file exists
    if let Err(_) = fs::metadata(arg.path.to_string()) {
        utils::dialog_file_not_found(arg.path.to_string());
        exit(255);
    }

    //let menu = Menu::new().add_item(1, "Test", Some(&HotKey::new(RawMods::CtrlMeta, "K")), Some(true), true);


    let monitor = Screen::get_monitors().first().unwrap().clone();

    let image = image::open(arg.path.to_string()).unwrap();

    let monitor_width = monitor.virtual_work_rect().width();
    let image_width = image.width() as f64;
    let image_height = image.height() as f64;

    let initial_state = AppState::new(
        image,
        1f64,
        arg.path.to_string(),
        monitor,
        Color::RED
    );

    initial_state.scale_factor.set( image_width / monitor_width + 0.5f64);
    let window_width = image_width / initial_state.scale_factor.get();
    let window_height = (image_height * window_width)/image_width;


    let main_window = WindowDesc::new(ui_builder())
        .title(format!("Screen Crab Tools [{}]", arg.path.to_string()))
        .window_size((window_width, window_height))
        .menu(|_, _, _| {
            menu::create_menu()
        });

    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(move |env, _| {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::TRANSPARENT);
            env.set(druid::theme::BUTTON_DARK, Color::WHITE);
            env.set(druid::theme::SCROLLBAR_MAX_OPACITY, 0);
            env.set(druid::theme::BUTTON_LIGHT, Color::WHITE);
            env.set(druid::theme::TEXT_COLOR, Color::BLACK);
        })
        .launch(initial_state)
}