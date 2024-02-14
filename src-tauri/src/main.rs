// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod service;
mod utils;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time::Duration};

use app_state::AppState;

use service::action::Action;
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn on_system_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
    state: &AppState,
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            dbg!(&id);

            match id.as_str() {
                "start" => {
                    app.tray_handle().get_item(&id).set_enabled(false).unwrap();
                    app.tray_handle()
                        .get_item("stop")
                        .set_enabled(true)
                        .unwrap();

                    let s = state.clone();

                    thread::spawn(move || {
                        let mut action = Action::new();
                        let mut next_time_flag: u64 =
                            utils::get_current_timestamp_in_millis() as u64;
                        loop {
                            let recv = receiver.lock().unwrap().try_recv();
                            match recv {
                                Ok(_) => {
                                    dbg!("terminated");
                                    break;
                                }
                                _ => {}
                            }
                            let current: u64 = utils::get_current_timestamp_in_millis() as u64;
                            dbg!(current);

                            if current < next_time_flag {
                                thread::sleep(Duration::from_millis(1000));
                                continue;
                            }

                            action.move_mouse();
                            next_time_flag = current + s.idle_time * 1000;
                        }
                    });
                }
                "stop" => {
                    sender
                        .lock()
                        .unwrap()
                        .send("terminate".to_string())
                        .unwrap();

                    app.tray_handle().get_item(&id).set_enabled(false).unwrap();
                    app.tray_handle()
                        .get_item("start")
                        .set_enabled(true)
                        .unwrap();
                }
                "quit" => app.exit(0),
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    let state = AppState::new(5);
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("start", "Start"))
        .add_item(CustomMenuItem::new("stop", "Stop").disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(move |app, event| {
            on_system_tray_event(app, event, &state, Arc::clone(&tx), Arc::clone(&rx));
        }) // Change closure to FnMut
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
