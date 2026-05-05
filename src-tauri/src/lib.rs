use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, WebviewWindowBuilder, WebviewUrl, Emitter};

struct AppState {
    current_state: Mutex<serde_json::Value>,
}

#[tauri::command]
fn get_state(state: State<AppState>) -> serde_json::Value {
    state.current_state.lock().unwrap().clone()
}

#[tauri::command]
fn set_state(state: serde_json::Value, app_state: State<AppState>, app: AppHandle) {
    *app_state.current_state.lock().unwrap() = state.clone();
    app.emit("state-changed", state).unwrap();
}

#[tauri::command]
async fn open_presentation_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("presentation") {
        window.set_focus().unwrap();
    } else {
        let _window = WebviewWindowBuilder::new(
            &app,
            "presentation",
            WebviewUrl::App("?view=presentation".into())
        )
        .title("Scripture Presentation")
        .inner_size(1024.0, 768.0)
        .build()
        .unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(AppState {
            current_state: Mutex::new(serde_json::json!({})),
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            set_state,
            open_presentation_window
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
