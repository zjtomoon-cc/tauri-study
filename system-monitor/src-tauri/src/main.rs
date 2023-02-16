#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod native;
pub mod monitor;

use native::{native_windows,create_tray};
use tauri::{AppHandle,Manager,SystemTrayEvent};

use crate::monitor::{
    battery_info,
    cpu_info,
    memory_info,
    process_info,
    system_info,
    update_tray_title
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let system_tray = create_tray();
    tauri::Builder::default().setup(|app| {
        let window = app.get_window("main").unwrap();
        native_windows(&window,Some(10.),false);
        Ok(())
    })
        .system_tray(system_tray)
        .on_system_tray_event(system_try_handler)
        .invoke_handler(tauri::generate_handler![
            system_info,
            battery_info,
            cpu_info,
            process_info,
            memory_info,
            update_tray_title,
        ]).run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/**
* 处理系统托盘菜单的点击事件
*
*/

fn system_try_handler(app: &AppHandle,event:SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position:pos,
            size:_,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.set_position(pos).unwrap();
            #[cfg(target_os = "windows")]
            window.center().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick {id,..} => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0)
                }
                _ => {}
            }
        }
        _ => {}
    }
}