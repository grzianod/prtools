#![windows_subsystem = "windows"]
mod utils;
mod painter;
mod menu;

use std::fs;
use std::process::{exit};
use druid::widget::{Align, Flex, Scroll};
use druid::{AppLauncher, Color, ImageBuf, PlatformError, Screen, Widget, WidgetExt, WindowDesc};
use clap::Parser;
use druid_shell::WindowState;
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
    let monitor_height = monitor.virtual_work_rect().height();
    let image_width = image.width() as f64;
    let image_height = image.height() as f64;

    let mut initial_state = AppState::new(
        image,
        1f64,
        arg.path.to_string(),
        monitor,
        Color::RED
    );

    initial_state.scale_factor.set( image_width / monitor_width + 0.5f64);
    let mut window_width = image_width / initial_state.scale_factor.get();
    let mut window_height = ((image_height * window_width)/image_width);


    let main_window = WindowDesc::new(ui_builder())
        .title(format!("Screen Crab Tools [{}]", arg.path.to_string()))
        .window_size((window_width, window_height))
        .menu(|id, data, env| {
            menu::create_menu()
        });

    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(move |env, _| {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::TRANSPARENT);
            env.set(druid::theme::BUTTON_DARK, druid::Color::WHITE);
            env.set(druid::theme::SCROLLBAR_MAX_OPACITY, 0);
            env.set(druid::theme::BUTTON_LIGHT, druid::Color::WHITE);
            env.set(druid::theme::TEXT_COLOR, druid::Color::BLACK);
        })
        .launch(initial_state)
}