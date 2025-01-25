use std::process::Command;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[tauri::command]
fn get_processor_name() -> String {
    let sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    let processor_name: String = sys.cpus()[0].brand().to_string();
    processor_name
}

#[tauri::command]
fn get_cpu_usage() -> String {
    let mut sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    let mut cpu_usage_average: f32 = 0.0;

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();

    for cpu in sys.cpus() {
        cpu_usage_average += cpu.cpu_usage();
    }

    cpu_usage_average = cpu_usage_average / sys.cpus().len() as f32;

    format!("{:.1}%", cpu_usage_average)
}

#[tauri::command]
#[cfg(target_os = "linux")]
fn get_cpu_temp() -> Result<String, String> {
    let output = Command::new("sensors")
        .output()
        .map_err(|e| e.to_string())?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.contains("Package id 0") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(temp) = parts.get(3) {
                return Ok(temp.replace("+", ""));
            }
        }
    }

    Err("Temp not found".to_string())
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn get_cpu_temp() -> Result<String, String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject MSAcpi_ThermalZoneTemperature | Select-Object CurrentTemperature")
        .output()
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
#[cfg(target_os = "macos")]
fn get_cpu_temp() -> Result<String, String> {
    let output = Command::new("powermetrics")
        .arg("--samplers")
        .arg("smc")
        .arg("-n")
        .arg("1")
        .output()
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn get_cpu_frequency() -> String {
    let sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    let mut cpu_frequency_average: f32 = 0.0;

    for cpu in sys.cpus() {
        cpu_frequency_average += cpu.frequency() as f32;
    }

    cpu_frequency_average = cpu_frequency_average / sys.cpus().len() as f32 / 1000.0;

    format!("{:.1} GHz", cpu_frequency_average)
}

#[tauri::command]
fn get_memory_usage() -> String {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );

    sys.refresh_memory();

    let ram_usage = sys.used_memory() / 1024 / 1024 / 1024;
    let ram_total = sys.total_memory() / 1024 / 1024 / 1024;

    format!("{}/{} GB", ram_usage, ram_total)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            app.get_webview_window("main").unwrap().set_always_on_top(true).unwrap();
            #[cfg(desktop)]
            {
                let _ = app.handle().plugin(
                    tauri_plugin_positioner::init()
                );

                let open_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyO);
                let hidden_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyI);
                let quit_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyQ);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            if shortcut == &open_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        let window =
                                            app.app_handle().get_webview_window("main").unwrap();
                                        window
                                            .set_position(tauri::Position::Logical(
                                                tauri::LogicalPosition { x: 1520.0, y: 0.0 },
                                            ))
                                            .unwrap();

                                        window.show().unwrap();
                                        window.set_focus().unwrap();
                                        dbg!("alt+o pressed");
                                    }
                                    ShortcutState::Released => {
                                        dbg!("alt+o released");
                                    }
                                }
                            } else if shortcut == &hidden_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        app.app_handle()
                                            .get_webview_window("main")
                                            .unwrap()
                                            .hide()
                                            .unwrap();
                                        dbg!("alt+i pressed");
                                    }
                                    ShortcutState::Released => {
                                        dbg!("alt+i released");
                                    }
                                }
                            } else if shortcut == &quit_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        app.app_handle().cleanup_before_exit();
                                        dbg!("alt+q pressed");
                                    }
                                    ShortcutState::Released => {
                                        app.app_handle().exit(0);
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(open_shortcut)?;
                app.global_shortcut().register(hidden_shortcut)?;
                app.global_shortcut().register(quit_shortcut)?;
            }

            let show = MenuItem::with_id(app, "show", "show", true, Some("CmdOrCtrl+1"));
            let hidden = MenuItem::with_id(app, "hidden", "hidden", true, Some("CmdOrCtrl+2"));
            let quit = MenuItem::with_id(app, "quit", "quit", true, Some("CmdOrCtrl+Q"));
            let menu = Menu::with_items(app, &[&show.unwrap(), &hidden.unwrap(), &quit.unwrap()])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window
                            .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                                x: 1520.0,
                                y: 0.0,
                            }))
                            .unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "hidden" => {
                        app.get_webview_window("main").unwrap().hide().unwrap();
                    }
                    "quit" => {
                        app.cleanup_before_exit();
                        app.exit(0);
                    }
                    _ => {
                        dbg!("unknown menu item");
                    }
                })
                .on_tray_icon_event(|tray_handle, event| {
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
                })
                .tooltip(app.package_info().name.clone())
                .build(app);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_processor_name,
            get_cpu_usage,
            get_cpu_temp,
            get_cpu_frequency,
            get_memory_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
