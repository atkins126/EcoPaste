use tauri::{command, utils::config::WindowConfig, AppHandle, Manager, Window, WindowBuilder};
use window_vibrancy::*;

// 主窗口的名称
pub static MAIN_WINDOW_LABEL: &str = "main";

// 创建窗口
#[command]
pub fn create_window(app_handle: AppHandle, label: String, mut options: WindowConfig) {
    let window = app_handle.get_window(&label);

    if window.is_some() {
        return show_window(window.clone().unwrap());
    }

    options.label = label.clone();

    Some(
        WindowBuilder::from_config(&app_handle, options.clone())
            .build()
            .expect("failed to create window"),
    )
    .unwrap();
}

// 显示窗口
#[command]
pub fn show_window(window: Window) {
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}

// 隐藏窗口
#[command]
pub fn hide_window(window: Window) {
    window.hide().unwrap();
}

// 退出 app
#[command]
pub fn quit_app() {
    std::process::exit(0)
}

// 磨砂窗口：https://github.com/tauri-apps/window-vibrancy
#[command]
pub fn frosted_window(window: Window) {
    #[cfg(target_os = "macos")]
    apply_vibrancy(
        &window,
        NSVisualEffectMaterial::HeaderView,
        Some(NSVisualEffectState::Active),
        Some(10.0),
    )
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}
