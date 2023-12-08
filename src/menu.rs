use std::fs;
use std::process::exit;
use druid::{Affine, Color, commands, Env, Event, ImageBuf};
use crate::utils;
use crate::utils::{Action, AppState, Selection};
use image::{DynamicImage, ImageBuffer, Rgba};
use notify_rust::Notification;
use druid::RawMods;

pub fn create_menu() -> druid::Menu<AppState> {

    #[cfg(target_os = "macos")]
    let about = druid::Menu::new(druid::LocalizedString::new("Screen Crab Tools"))
        .entry(druid::MenuItem::new("About Screen Crab Tools").command(commands::SHOW_ABOUT))
        .separator()
        .entry(druid::MenuItem::new("Hide Screen Crab Tools").hotkey(Some(RawMods::Meta), "H").command(commands::HIDE_APPLICATION))
        .entry(druid::MenuItem::new("Hide Others").hotkey(Some(RawMods::AltMetaShift), "H").command(commands::HIDE_OTHERS))
        .separator()
         .entry(druid::MenuItem::new("Quit Screen Crab Tools").hotkey(Some(RawMods::Meta), "Q").command(commands::QUIT_APP));

    let file = druid::Menu::new(druid::LocalizedString::new("File"))
        .entry(druid::platform_menus::common::paste())
        .separator()
        .entry(druid::MenuItem::new("Save").hotkey(Some(RawMods::Meta), "S")
            .on_activate( move |ctx, data: &mut AppState, env| {
                data.save.set(true);
                data.repaint = true;
            })
        )
        .entry(druid::MenuItem::new("Delete").hotkey(Some(RawMods::Meta), "D")
            .on_activate(move |ctx, data: &mut AppState, env| {
                if utils::dialog_delete_file(data.image_path.to_string()) {
                    fs::remove_file(data.image_path.to_string()).unwrap();
                    exit(0);
                }
            })
        );


    let tools = druid::Menu::new(druid::LocalizedString::new("Tools"))
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
        .entry(druid::Menu::new(druid::LocalizedString::new("Shapes"))
                   .entry(druid::MenuItem::new("Fill")
                       .selected_if(|data: &AppState, env| {
                           data.fill_color == true
                       })
                       .on_activate(|ctx, data: &mut AppState, env| {
                            data.fill_color = !data.fill_color
                       }))
                   .separator()
                   .entry(druid::menu::MenuItem::new("Rectangle")
                       .selected_if(|data: &AppState, env| {
                           data.selection.eq(&Selection::Rectangle)
                       })
                       .on_activate(|ctx, data: &mut AppState, env| {
                           data.selection = Selection::Rectangle;

                       }))
                   .entry(druid::menu::MenuItem::new("Circle")
                       .selected_if(|data: &AppState, env| {
                           data.selection.eq(&Selection::Circle)
                       }).on_activate(
                       |ctx, data: &mut AppState, env| {
                           data.selection = Selection::Circle;
                       }
                   ))
                   .entry(druid::menu::MenuItem::new("Ellipse")
                       .selected_if(|data: &AppState, env| {
                           data.selection.eq(&Selection::Ellipse)
                       })
                       .on_activate(|ctx, data: &mut AppState, env| {
                           data.selection = Selection::Ellipse;
                       }
                   ))  // TBI
        )
        .entry( druid::MenuItem::new("Arrow").hotkey(Some(RawMods::Meta), "W")
            .on_activate(|ctx, data: &mut AppState, env| {
                data.selection = Selection::Arrow;
            })
            .selected_if(|data: &AppState, env| {
                data.selection.eq(&Selection::Arrow)
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
            .entry(druid::MenuItem::new("Pick a color...")
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.is_picking_color = true;
                }))
            .entry(druid::MenuItem::new(|data: &AppState, env: &_| {
                format!("#{:02X}{:02X}{:02X}", data.color.as_rgba8().0, data.color.as_rgba8().1, data.color.as_rgba8().2)
            })
                .enabled_if(|data: &AppState, env| {
                    false
                })
                .selected_if(|data: &AppState, env| {
                    data.custom_color == true
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::RED;
                }))
            .separator()
            .entry(druid::MenuItem::new("Red")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::RED)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::RED;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Green")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::GREEN)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::GREEN;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Black")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::BLACK)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::BLACK;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("White")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::WHITE)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::WHITE;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Aqua")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::AQUA)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::AQUA;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Blue")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::BLUE)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::BLUE;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Fuchsia")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::FUCHSIA)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::FUCHSIA;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Gray")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::GRAY)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::GRAY;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Maroon")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::MAROON)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::MAROON;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Yellow")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::YELLOW)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::YELLOW;
                    data.custom_color = false;
                }))
            .entry(druid::MenuItem::new("Silver")
                .selected_if(|data: &AppState, env| {
                    data.color.eq(&Color::SILVER)
                })
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.color = Color::SILVER;
                    data.custom_color = false;
                }))
        )
        .entry(druid::Menu::new(druid::LocalizedString::new("Stroke"))
            .entry(druid::MenuItem::new("2 pt")
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.stroke = 2.0;
                })
                .selected_if(|data: &AppState, env| {
                    data.stroke == 2.0
                })
            )
            .entry(druid::MenuItem::new("3 pt")
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.stroke = 3.0;
                })
                .selected_if(|data: &AppState, env| {
                    data.stroke == 3.0
                })
            )
            .entry(druid::MenuItem::new("5 pt")
                .on_activate(|ctx, data: &mut AppState, env| {
                    data.stroke = 5.0;
                })
                .selected_if(|data: &AppState, env| {
                    data.stroke == 5.0
                })
            )
        )
        .entry(druid::Menu::new(druid::LocalizedString::new("Font Size"))
            .entry(druid::MenuItem::new("Normal")
                .on_activate(|ctx, data: &mut AppState, env| {

                })
                .selected_if(|data: &AppState, env| {
                    false
                })
            )
            .entry(druid::MenuItem::new("Large")
                .on_activate(|ctx, data: &mut AppState, env| {

                })
                .selected_if(|data: &AppState, env| {
                        false
                })
            )
        );

    let actions =  druid::Menu::new(druid::LocalizedString::new("Actions"))
        .entry(druid::MenuItem::new(|data: &AppState, env: &Env| {
            if let Some(last) = data.actions.last() {
                match last {
                    Action::Pen(_, _, _, _) => { return format!("Undo Pen"); }
                    Action::Highlighter(_, _, _, _) => { return format!("Undo Highlighter"); }
                    Action::Arrow(_, _, _, _, _) => { return format!("Undo Arrow"); }
                    Action::Rectangle(_, _, _, _, _, _) => { return format!("Undo Rectangle"); }
                    Action::Circle(_, _, _, _, _, _) => { return format!("Undo Circle"); }
                    Action::Ellipse(_, _, _, _, _, _) => { return format!("Undo Ellipse"); }
                    Action::Text(_, _, _, _) => { return format!("Undo Text"); }
                    Action::Crop(_, _, _) => { return format!("Undo Crop"); }
                }
            }
            else { return "Undo".to_string(); }
        }).hotkey(Some(RawMods::Meta), "Z")
            .on_activate(|ctx, data: &mut AppState, env| {
                if let Some(action) =  data.actions.pop() {
                    data.redo_actions.push(action);
                }
                data.repaint = true;
            })
            .enabled_if(|data: &AppState, env| {
                data.actions.len() > 0
            })
        )
        .entry(druid::MenuItem::new(|data: &AppState, env: &Env| {
            if let Some(last) = data.redo_actions.last() {
                match last {
                    Action::Pen(_, _, _, _) => { return format!("Redo Pen"); }
                    Action::Highlighter(_, _, _, _) => { return format!("Redo Highlighter"); }
                    Action::Arrow(_, _, _, _, _) => { return format!("Redo Arrow"); }
                    Action::Rectangle(_, _, _, _, _, _) => { return format!("Redo Rectangle"); }
                    Action::Circle(_, _, _, _, _, _) => { return format!("Redo Circle"); }
                    Action::Ellipse(_, _, _, _, _, _) => { return format!("Redo Ellipse"); }
                    Action::Text(_, _, _, _) => { return format!("Redo Text"); }
                    Action::Crop(_, _, _) => { return format!("Redo Crop"); }
                }
            }
            else { return "Redo".to_string(); }
        }).hotkey(Some(RawMods::MetaShift), "Z")
            .on_activate(|ctx, data: &mut AppState, env| {
                if let Some(redo_action) = data.redo_actions.pop() {
                    data.actions.push(redo_action);
                }
                data.repaint = true;
            })
            .enabled_if(|data: &AppState, env| {
                data.redo_actions.len() > 0
            })
        )
        .separator()
        .entry(druid::MenuItem::new("Crop").hotkey(Some(RawMods::Meta), "K")
            .on_activate(|ctx, data: &mut AppState, env| {
                data.selection = Selection::Crop;
                data.crop.set(true);
                data.repaint = true;
            }))
        .entry(druid::MenuItem::new("Rotate").hotkey(Some(RawMods::Meta), "R")
            .on_activate(|ctx, data: &mut AppState, env| {
                data.repaint = true;
            }))
        .entry(druid::MenuItem::new("Flip Vertical").hotkey(Some(RawMods::Meta), "L")
            .on_activate(|ctx, data: &mut AppState, env| {
                if data.affine == Affine::FLIP_Y {
                    data.affine = Affine::IDENTITY;
                }
                else {
                    data.affine = Affine::FLIP_Y;
                }
                data.repaint = true;
            }))
        .entry(druid::MenuItem::new("Flip Horizontal").hotkey(Some(RawMods::Meta), "I")
            .on_activate(|ctx, data: &mut AppState, env| {
                if data.affine == Affine::FLIP_X {
                    data.affine = Affine::IDENTITY;
                }
                else {
                    data.affine = Affine::FLIP_X;
                }
                data.repaint = true;
                data.repaint = true;
            }));


    #[cfg(target_os = "macos")] {
        return druid::Menu::empty()
            .entry(about)
            .entry(file)
            .entry(tools)
            .entry(actions);
    }

    #[cfg(not(target_os="macos"))] {
        return druid::Menu::empty()
            .entry(file)
            .entry(tools)
            .entry(actions);
    }
}