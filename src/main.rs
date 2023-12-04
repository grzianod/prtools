use std::fs;
use std::process::exit;
use druid::widget::{Align, Button, FillStrat, Flex, Image, Label};
use druid::{AppLauncher, ImageBuf, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, WindowHandle, WindowLevel};
use druid::{Data, Lens};
use clap::Parser;
use druid::Value::Size;
use image::{DynamicImage};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::Path;

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

fn ui_builder(path: String, image: DynamicImage) -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let width = image.width();
    let height = image.height();
    let img = Image::new(ImageBuf::from_dynamic_image(image)).fix_size((width as f64)/3f64, (height as f64)/3f64);
    let pen = Button::new("Pen️").padding(5.0);
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
        .padding(5.0);
    /*let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut u32, _env| *data += 1)
        .padding(5.0);

    let pen = Button::new("pen")
        .on_click(|_ctx, data: &mut u32, _env| *data+=1)
        .padding(5.0);*/

    let first_row = Flex::row().with_child(pen).with_child(zoom_in).with_child(zoom_out).with_child(fit).padding(10.0);
    let img_row = Flex::row().with_child(img).padding(10.0);
    let second_row = Flex::row().with_child(save).with_child(delete).padding(10.0);
    let container = Flex::column().with_child(first_row).with_child(img_row).with_child(second_row);
    Align::centered(container)
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
        .window_size((width as f64, height as f64));

    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}