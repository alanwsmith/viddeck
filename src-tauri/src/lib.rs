use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      use tauri_plugin_global_shortcut::{
        Code, GlobalShortcutExt, Modifiers, Shortcut,
        ShortcutState,
      };

      let webview_window =
        tauri::WebviewWindowBuilder::new(
          app,
          "main",
          tauri::WebviewUrl::App(
            "https://www.youtube.com".into(),
          ),
        )
        .build()?;
      webview_window.eval(
        r#"
function awsPlayPause() {
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.paused) {
      if (vid.currentTime > 10) {
        vid.currentTime -= 7;
      }
      vid.play();
    }
    else {
      vid.pause()
    }
  }
}

function awsRewind() {
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.currentTime > 13) {
      vid.currentTime -= 10;
    }
    else {
      vid.currentTime = 0;
    }
  }
}

function awsFastForward() {
  let vid = document.querySelector("video");
  if (vid) {
    vid.currentTime += 8;
  }
}


function awsPlayFaster() {
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.playbackRate < 4.0) {
      vid.playbackRate += 0.25;
    }
  }
}


function awsPlaySlower() {
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.playbackRate > 0.25) {
      vid.playbackRate -= 0.25;
    }
  }
}

"#,
      )?;

      #[cfg(debug_assertions)]
      webview_window.open_devtools();

      let play_pause_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyS,
      );

      app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
          .with_handler(move |appx, shortcut, event| {
            if shortcut == &play_pause_key {
              if let ShortcutState::Pressed =
                event.state()
              {
                appx
                  .get_webview_window("main")
                  .expect("no main widow")
                  .eval(r#"awsPlayPause();"#)
                  .expect("eval did not work");
              }
              // match event.state() {
              //   ShortcutState::Pressed => {
              //     let wv = appx
              //       .get_webview_window("main")
              //       .expect("no main widow");
              //     wv.eval(r#"awsPlayPause();"#)
              //       .expect("eval did not work");
              //   }
              //   _ => (),
              // }
            }
          })
          .build(),
      )?;
      app.global_shortcut().register(play_pause_key)?;
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

// // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//   format!(
//     "Hello, {}! You've been greeted from Rust!",
//     name
//   )
// }
