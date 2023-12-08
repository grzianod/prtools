use std::cell::Cell;
use std::path::Path;
use druid::{Affine, Color, ImageBuf, Monitor, Point, Size};
use druid::{Data, Lens};
use clap::Parser;
use image::DynamicImage;
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
    Rectangle,
    Circle,
    Ellipse,
    Arrow,
    Text,
    Crop,
}

impl Default for Selection {
    fn default() -> Self {
        return Self::Pen
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Pen(Affine, Vec<Point>, Color, f64),
    Highlighter(Affine, Vec<Point>, Color, f64),
    Rectangle(Affine, Point, Point, Color, bool, f64), // Stores rectangle points and color
    Circle(Affine, Point, f64, Color, bool, f64), // Stores circle points and color
    Ellipse(Affine, Point, Point, Color, bool, f64), // Stores ellipse points and color
    Arrow(Affine, Point, Point, Color, f64), // Stores arrow points and color
    Text(Affine, Point, String, Color),  // Stores position, text, and color
    Crop(DynamicImage, Point, Point),
}


impl Action {
    pub fn new(selection: &Selection) -> Self {
        match selection {
            Selection::Pen => Self::Pen(Affine::IDENTITY, Vec::new(), Color::RED, 2.0),
            Selection::Highlighter => Self::Highlighter(Affine::IDENTITY,Vec::new(), Color::RED, 2.0),
            Selection::Rectangle => Self::Rectangle(Affine::IDENTITY,Point::ZERO, Point::ZERO, Color::RED, false, 2.0),
            Selection::Circle => Self::Circle(Affine::IDENTITY,Point::ZERO, 0.0, Color::RED, false,2.0),
            Selection::Ellipse => Self::Ellipse(Affine::IDENTITY,Point::ZERO, Point::ZERO, Color::RED, false, 2.0),
            Selection::Arrow => Self::Arrow(Affine::IDENTITY,Point::ZERO, Point::ZERO, Color::RED, 2.0),
            Selection::Text => Self::Text(Affine::IDENTITY,Point::ZERO, String::from("test") ,Color::RED),
            Selection::Crop => Self::Crop(DynamicImage::default(), Point::ZERO, Point::ZERO),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Transformation {
    IDENTITY,
    FLIP_V,
    FLIP_H,
    ROTATE(f64)
}


#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    #[data(same_fn = "PartialEq::eq")]
    pub affine: Affine,
    #[data(same_fn = "PartialEq::eq")]
    pub selection: Selection,
    #[data(same_fn = "PartialEq::eq")]
    pub image: DynamicImage,
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
    pub fill_color: bool,
    pub stroke: f64,
    pub is_writing_text: bool,
    #[data(same_fn = "PartialEq::eq")]
    pub save: Cell<bool>,
    #[data(same_fn = "PartialEq::eq")]
    pub update: Cell<bool>,
    pub zoom: f64,
    #[data(same_fn = "PartialEq::eq")]
    pub crop: Cell<bool>
}

impl AppState {
    pub fn new(image: DynamicImage, image_path: String, monitor: Monitor, color: Color) -> Self {
        AppState {
            affine: Affine::IDENTITY,
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
            custom_color: false,
            fill_color: false,
            stroke: 2.0,
            is_writing_text: false,
            update: Cell::new(false),
            zoom: 1f64,
            save: Cell::new(false),
            crop: Cell::new(false),
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