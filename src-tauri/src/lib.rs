use std::{thread, time::Duration};
use tauri::{menu::{Menu, MenuItem, PredefinedMenuItem, Submenu}, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|handle| 
            Menu::with_items(
                handle,
                &[
                    &Submenu::with_items(
                        handle,
                        "File",
                        true,
                        &[
                            &PredefinedMenuItem::about(handle, None, Default::default())?,
                            &PredefinedMenuItem::separator(handle)?,
                            &PredefinedMenuItem::hide(handle, None)?,
                            &PredefinedMenuItem::hide_others(handle, None)?,
                            &PredefinedMenuItem::separator(handle)?,
                            &MenuItem::with_id(handle, "signOut", "Sign Out", true, Some("Sign Out"))?,
                            &PredefinedMenuItem::quit(handle, None)?,
                        ],
                    )?,
                    &Submenu::with_items(
                        handle,
                        "Edit",
                        true,
                        &[
                            &PredefinedMenuItem::undo(handle, None)?,
                            &PredefinedMenuItem::redo(handle, None)?,
                            &PredefinedMenuItem::separator(handle)?,
                            &PredefinedMenuItem::cut(handle, None)?,
                            &PredefinedMenuItem::copy(handle, None)?,
                            &PredefinedMenuItem::paste(handle, None)?,
                            &PredefinedMenuItem::select_all(handle, None)?,
                        ],
                    )?,
                    &Submenu::with_items(
                        handle,
                        "Window",
                        true,
                        &[
                            &PredefinedMenuItem::minimize(handle, None)?,
                            &PredefinedMenuItem::fullscreen(handle, None)?,
                            &PredefinedMenuItem::separator(handle)?,
                            &PredefinedMenuItem::close_window(handle, None)?
                        ],
                    )?,
                ],
            )
        )
        .on_menu_event(|app, event| {
            if event.id() == "signOut" {
                app.get_webview_window("main").unwrap().eval("window.location.href = 'https://mail.google.com/mail/logout?hl=en';").unwrap();
                let _ = thread::spawn(|| {
                    thread::sleep(Duration::from_millis(500));
                    std::process::exit(0x0);
                });
            }
         })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
