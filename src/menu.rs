use std::fs;
use std::process::exit;
use druid::{Color, ImageBuf, WidgetExt};
use druid::Target::Widget;
use druid_shell::RawMods;
use crate::utils;
use crate::utils::{AppState, Selection};

pub fn create_menu() -> druid::Menu<AppState> {
    druid::Menu::empty()
        .entry(
            druid::Menu::new(druid::LocalizedString::new("Screen Crab Tools"))
                .entry(druid::platform_menus::mac::application::about())
                .separator()
                .entry(druid::MenuItem::new("Save").hotkey(Some(RawMods::Meta), "S"))
                .entry(druid::MenuItem::new("Delete").hotkey(Some(RawMods::Meta), "D")
                    .on_activate(move |ctx, data: &mut AppState, env| {
                        if utils::dialog_delete_file(data.image_path.to_string()) {
                            fs::remove_file(data.image_path.to_string()).unwrap();
                            exit(0);
                        }
                    })
                )
                .separator()
                .entry(druid::platform_menus::mac::application::hide())
                .entry(druid::platform_menus::mac::application::hide_others())
                .separator()
                .entry(druid::platform_menus::mac::application::quit())
        )
        .entry(
            druid::Menu::new(druid::LocalizedString::new("Tools"))
                .entry(druid::MenuItem::new("Pen").hotkey(Some(RawMods::Meta), "P")
                    .selected_if(|data: &AppState, env| {
                        data.selection.eq(&Selection::Pen)
                    })
                    .on_activate(|ctx, data: &mut AppState, env| {
                        data.selection = Selection::Pen;
                    }))
                .entry(druid::MenuItem::new("Highlighter").hotkey(Some(RawMods::Meta), "H")
                    .selected_if(|data: &AppState, env| {
                        data.selection.eq(&Selection::Highlighter)
                    })
                    .on_activate(|ctx, data: &mut AppState, env| {
                        data.selection = Selection::Highlighter;
                    }))
                .entry(druid::MenuItem::new("Text").hotkey(Some(RawMods::Meta), "T")
                    .selected_if(|data: &AppState, env| {
                        data.selection.eq(&Selection::Text)
                    })
                    .on_activate(|ctx, data: &mut AppState, env| {
                        data.selection = Selection::Text;
                    }))
                .separator()
                .entry(druid::Menu::new(druid::LocalizedString::new("Color"))
                    .entry(druid::MenuItem::new("Red")
                        .selected_if(|data: &AppState, env| {
                            data.color.eq(&Color::RED)
                        })
                        .on_activate(|ctx, data: &mut AppState, env| {
                            data.color = Color::RED;
                        }))
                    .entry(druid::MenuItem::new("Green")
                        .selected_if(|data: &AppState, env| {
                            data.color.eq(&Color::GREEN)
                        })
                        .on_activate(|ctx, data: &mut AppState, env| {
                            data.color = Color::GREEN;
                        }))
                    .entry(druid::MenuItem::new("Black")
                        .selected_if(|data: &AppState, env| {
                            data.color.eq(&Color::BLACK)
                        })
                        .on_activate(|ctx, data: &mut AppState, env| {
                            data.color = Color::BLACK;
                        }))
                    .entry(druid::MenuItem::new("White")
                        .selected_if(|data: &AppState, env| {
                            data.color.eq(&Color::WHITE)
                        })
                        .on_activate(|ctx, data: &mut AppState, env| {
                            data.color = Color::WHITE;
                        }))
                )
        )
        .entry(
            druid::Menu::new(druid::LocalizedString::new("Actions"))
                .entry(druid::platform_menus::common::undo()
                    .on_activate(|ctx, data: &mut AppState, env| {
                        if let Some(action) =  data.actions.pop() {
                            data.redo_actions.push(action);
                        }
                    }))
                .entry(druid::platform_menus::common::redo()
                    .on_activate(|ctx, data: &mut AppState, env| {
                        if let Some(redo_action) = data.redo_actions.pop() {
                            data.actions.push(redo_action);
                        }
                    }))
                .separator()
                .entry(druid::MenuItem::new("Rotate").hotkey(Some(RawMods::Meta), "R")
                    .on_activate(|ctx, data: &mut AppState, env| {
                        let image = image::open(data.image_path.to_string()).unwrap();
                        data.image = ImageBuf::from_dynamic_image(image.rotate90());
                    }))
                .entry(druid::MenuItem::new("Flip Vertical").hotkey(Some(RawMods::Meta), "L")
                    .on_activate(|ctx, data: &mut AppState, env| {
                        let image = image::open(data.image_path.to_string()).unwrap();
                        data.image = ImageBuf::from_dynamic_image(image.flipv());
                    }))
                .entry(druid::MenuItem::new("Flip Horizontal").hotkey(Some(RawMods::Meta), "I")
                    .on_activate(|ctx, data: &mut AppState, env| {
                        let image = image::open(data.image_path.to_string()).unwrap();
                        data.image = ImageBuf::from_dynamic_image(image.fliph());
                    }))
        )
}