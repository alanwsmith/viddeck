// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!(
    "Hello, {}! You've been greeted from Rust!",
    name
  )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let webview_window =
        tauri::WebviewWindowBuilder::new(
          app,
          "label",
          tauri::WebviewUrl::App(
            "https://www.youtube.com".into(),
          ),
        )
        .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  // tauri::Builder::default()
  //     .plugin(tauri_plugin_global_shortcut::Builder::new().build())
  //     .plugin(tauri_plugin_opener::init())
  //     .invoke_handler(tauri::generate_handler![greet])
  //     .run(tauri::generate_context!())
  //     .expect("error while running tauri application");
}
