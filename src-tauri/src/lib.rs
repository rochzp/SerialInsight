#[cfg(target_os = "macos")]
use objc2_app_kit::{NSColor, NSWindow};
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{WebviewUrl, WebviewWindowBuilder};
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("")
                .inner_size(1200.0, 800.0);
            #[cfg(not(target_os = "macos"))]
            {
                win_builder.decorations(false).build().unwrap();
            }

            // set background color only when building for macOS
            #[cfg(target_os = "macos")]
            {
                let win_builder = win_builder
                    .title_bar_style(TitleBarStyle::Overlay)
                    .transparent(true);
                let window = win_builder.build().unwrap();
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
                unsafe {
                    let ns_window: &NSWindow = &*window.ns_window().unwrap().cast();
                    let bg_color = NSColor::colorWithRed_green_blue_alpha(
                        50.0 / 255.0,
                        158.0 / 255.0,
                        163.5 / 255.0,
                        0.0,
                    );
                    ns_window.setBackgroundColor(Some(&bg_color));
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
