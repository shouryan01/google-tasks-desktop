#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let main_menu = Submenu::new(
        "File",
        Menu::new()
            .add_native_item(MenuItem::About(
                "Google Tasks".to_string(),
                Default::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("SignOut", "Sign Out"))
            .add_native_item(MenuItem::Quit),
    );
    let window_menu = Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::EnterFullScreen)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::CloseWindow),
    );
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let menu = Menu::new()
        .add_submenu(main_menu)
        .add_submenu(edit_menu)
        .add_submenu(window_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "SignOut" => {
                let _ = event
                    .window()
                    .eval("window.location.href = 'https://mail.google.com/mail/logout?hl=en';");

                let _ = thread::spawn(|| {
                    thread::sleep(Duration::from_millis(500));
                    std::process::exit(0x0);
                });
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
