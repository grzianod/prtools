#![windows_subsystem = "windows"]
mod utils;
mod painter;
mod menu;

use std::fs;
use std::process::{exit};
use druid::widget::{Align, Button, Flex, Image, Label, Scroll};
use druid::{AppLauncher, Color, ImageBuf, LocalizedString, PlatformError, Screen, TextAlignment, Widget, WidgetExt, WindowConfig, WindowDesc, WindowLevel};
use druid::{Data, Lens};
use clap::Parser;
use clap::Args;
use druid::RenderContext;
use druid::{LensExt};
use druid::piet::{Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use crate::utils::{AppState, Selection};
use crate::painter::DrawingWidget;
use std::io::Read;
use druid_shell::{HotKey, Menu, RawMods, SysMods};


fn ui_builder() -> impl Widget<AppState> {

    let drawing = Flex::row().with_child(DrawingWidget).padding(5.0);

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

    let image = ImageBuf::from_file(arg.path.to_string()).unwrap();
    let image_width = image.width();
    let image_height = image.height();

    let main_window = WindowDesc::new(ui_builder())
        .title(format!("Screen Crab Tools [{}]", arg.path.to_string()))
        .window_size((image_width as f64,image_height as f64))
        .menu(|id, data, env| {
            menu::create_menu()
        });


    let initial_state = AppState::new(
        image,
        arg.path.to_string(),
        monitor,
        Color::RED
    );
    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(|env, _| {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, druid::Color::WHITE);
            env.set(druid::theme::BUTTON_DARK, druid::Color::WHITE);
            env.set(druid::theme::SCROLLBAR_MAX_OPACITY, 0);
            env.set(druid::theme::BUTTON_LIGHT, druid::Color::WHITE);
            env.set(druid::theme::TEXT_COLOR, druid::Color::BLACK);
        })
        .launch(initial_state)
}