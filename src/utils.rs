use std::cell::Cell;
use std::path::Path;
use druid::{Color, ImageBuf, Monitor, Point, Widget, WidgetExt};
use druid::{Data, Lens};
use clap::Parser;
use druid::RenderContext;
use druid::{LensExt};
use druid::piet::{CoreGraphicsImage, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use tauri_dialog::DialogSelection;

/// Annotation Tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub path: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Selection {
    Pen,
    Highlighter,
    Text
}

impl Default for Selection {
    fn default() -> Self {
        return Self::Pen
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Pen(Vec<Point>, Cell<Color>),
    Highlighter(Vec<Point>, Cell<Color>),
    Text(String, Cell<Color>)
}

impl Action {
    pub fn new(selection: &Selection) -> Self {
        match selection {
            Selection::Pen => { Self::Pen(Vec::new(), Cell::new(Color::RED)) }
            Selection::Highlighter => { Self::Highlighter(Vec::new(), Cell::new(Color::RED)) }
            Selection::Text => { Self::Text(String::new(), Cell::new(Color::RED)) }
        }
    }
}


#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    #[data(same_fn = "PartialEq::eq")]
    pub selection: Selection,
    pub image: ImageBuf,
    #[data(same_fn = "PartialEq::eq")]
    pub actions: Vec<Action>,
    #[data(same_fn = "PartialEq::eq")]
    pub redo_actions: Vec<Action>,
    pub is_drawing: bool,
    pub image_path: String,
    #[data(same_fn = "PartialEq::eq")]
    pub monitor: Monitor,
    pub color: Color,
    pub repaint: bool,
    pub is_picking_color: bool,
    pub custom_color: bool,
}

impl AppState {
    pub fn new(image: ImageBuf, image_path: String, monitor: Monitor, color: Color) -> Self {
        AppState {
            selection: Selection::default(),
            image,
            actions: Vec::<Action>::new(),
            redo_actions: Vec::<Action>::new(),
            is_drawing: false,
            image_path,
            monitor,
            color,
            repaint: false,
            is_picking_color: false,
            custom_color: false
        }
    }
}

pub fn dialog_file_not_found(path: String) {
    tauri_dialog::DialogBuilder::new()
        .title("File Not Found!")
        .message(&format!("No such file \"{}\".\nPlease check that the file exists and try again.", Path::new(path.as_str()).file_name().unwrap().to_str().unwrap()))
        .style(tauri_dialog::DialogStyle::Error)
        .buttons(tauri_dialog::DialogButtons::Quit)
        .build()
        .show();
}

pub fn dialog_delete_file(path: String) -> bool {
    tauri_dialog::DialogBuilder::new()
        .title(&format!("Are you sure you want to delete {}", path ))
        .message(&format!("Tools will be closed and this item will be moved to Bin."))
        .style(tauri_dialog::DialogStyle::Question)
        .buttons(tauri_dialog::DialogButtons::YesNo)
        .build()
        .show()
        .eq(&DialogSelection::Yes)
}