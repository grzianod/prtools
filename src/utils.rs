use native_dialog::{MessageDialog, MessageType};
use std::path::Path;
use druid::{ImageBuf, Monitor, Widget, WidgetExt};
use druid::{Data, Lens};
use clap::Parser;
use druid::RenderContext;
use druid::{LensExt};
use druid::kurbo::{BezPath};
use druid::piet::{Text, TextLayoutBuilder};
use druid::widget::prelude::*;

/// Annotation Tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub path: String,
}
#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    pub image: ImageBuf,
    #[data(same_fn = "PartialEq::eq")]
    pub drawing_points: BezPath,
    pub is_drawing: bool,
    pub image_path: String,
    #[data(same_fn = "PartialEq::eq")]
    pub monitor: Monitor
}

impl AppState {
    pub fn new(image: ImageBuf, image_path: String, monitor: Monitor) -> Self {
        AppState { image, drawing_points: BezPath::new(), is_drawing: false, image_path, monitor}
    }
}

pub fn dialog_file_not_found(path: String) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(&format!("File Not Found!"))
        .set_text(&format!("No such file \"{}\".\nPlease check that the file exists and try again.", Path::new(path.as_str()).file_name().unwrap().to_str().unwrap()))
        .show_alert()
        .unwrap();
}

pub fn dialog_delete_file(path: String) -> bool {
    MessageDialog::new()
        .set_type(MessageType::Warning)
        .set_title(&format!("Are you sure you want to delete {}", path ))
        .set_text(&format!("Tools will be closed and this item will be moved to Bin."))
        .show_confirm()
        .unwrap()
}