#![windows_subsystem = "windows"]
mod utils;
mod painter;

use std::fs;
use std::process::{exit};
use druid::widget::{Align, Button, Flex, Image, Label, Scroll};
use druid::{AppLauncher, Color, ImageBuf, PlatformError, Screen, TextAlignment, Widget, WidgetExt, WindowConfig, WindowDesc, WindowLevel};
use druid::{Data, Lens};
use clap::Parser;
use clap::Args;
use druid::RenderContext;
use druid::{LensExt};
use druid::piet::{Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use image::{DynamicImage, ImageFormat, RgbImage};
use crate::utils::{AppState, Selection};
use crate::painter::DrawingWidget;
use std::io::Read;


fn ui_builder() -> impl Widget<AppState> {

    // The label text will be computed dynamically based on the current locale and count
    let pen = Button::new("🖊️").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            data.selection = Selection::Pen;
        }).fix_height(50.0);;
    let highlighter = Button::new("🖍").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            data.selection = Selection::Highlighter;
        }).fix_height(50.0);;
    let text = Button::new("Aa").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            data.selection = Selection::Text;
        }).fix_height(50.0);;
    let undo = Button::new("⬅️️").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            if let Some(action) =  data.actions.pop() {
                data.redo_actions.push(action);
            }
        }).fix_height(50.0);
    let redo = Button::new("➡️").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            if let Some(redo_action) = data.redo_actions.pop() {
                data.actions.push(redo_action);
            }
        }).fix_height(50.0);;
    let save = Button::new("Save").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            /* TODO */
    }).fix_height(50.0);;
    let delete = Button::new("Delete")
        .on_click(move |ctx, data: &mut AppState, env| {
            if utils::dialog_delete_file(data.image_path.to_string()) {
                fs::remove_file(data.image_path.to_string()).unwrap();
                exit(0);
            }
        })
        .padding(5.0).fix_height(50.0);;
    let flipv = Button::new("🔃").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            let image = image::open(data.image_path.to_string()).unwrap();
            data.image = ImageBuf::from_dynamic_image(image.flipv());
            ctx.request_paint();
    }).fix_height(50.0);;
    let fliph = Button::new("🔁").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            let image = image::open(data.image_path.to_string()).unwrap();
            data.image = ImageBuf::from_dynamic_image(image.fliph());
            ctx.request_paint();
    }).fix_height(50.0);;
    let rotate = Button::new("⤵️").padding(5.0)
        .on_click(|ctx, data: &mut AppState, env| {
            let image = image::open(data.image_path.to_string()).unwrap();
            data.image = ImageBuf::from_dynamic_image(image.rotate90());
            ctx.request_paint();
        }).fix_height(50.0);;
    let blank = Button::new("");

    let tools = Flex::row()
        .with_child(pen)
        .with_child(highlighter)
        .with_child(text)
        .with_child(flipv)
        .with_child(fliph)
        .with_child(rotate)
        .with_child(undo)
        .with_child(redo)
        .padding(5.0);
    let first_row = Flex::column().with_child(tools).padding(5.0);
    let drawing_row = Flex::row().with_child(DrawingWidget).padding(5.0);
    let second_row = Flex::row().with_child(save).with_child(delete).with_child(blank).padding(5.0);
    let container = Flex::column().with_child(first_row).with_child(drawing_row).with_child(second_row);
    Align::centered(Scroll::new(container))
}

fn main() -> Result<(), PlatformError> {
    let arg = utils::Args::parse();
    //check if the file exists
    if let Err(_) = fs::metadata(arg.path.to_string()) {
        utils::dialog_file_not_found(arg.path.to_string());
        exit(255);
    }

    let monitor = Screen::get_monitors().first().unwrap().clone();

    let image = ImageBuf::from_file(arg.path.to_string()).unwrap();
    let image_width = image.width();
    let image_height = image.height();

    let main_window = WindowDesc::new(ui_builder())
        .title(format!("Screen Crab Tools [{}]", arg.path.to_string()))
        .window_size((image_width as f64,image_height as f64));

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