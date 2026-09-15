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
  console.log("awsPlayPause()");
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.paused) {
      vid.currentTime = Math.max(0, vid.currentTime - 7);
      vid.play();
    }
    else {
      vid.pause()
    }
  }
};

function awsRewind() {
  console.log("awsRewind()");
  let vid = document.querySelector("video");
  if (vid) {
    vid.currentTime = Math.max(0, vid.currentTime - 10);
  }
};

function awsFastForward() {
  console.log("awsFastForward()");
  let vid = document.querySelector("video");
  if (vid) {
    vid.currentTime += 8;
  }
};


function awsPlayFaster() {
  console.log("awsPlayFaster()");
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.playbackRate < 4.0) {
      vid.playbackRate += 0.25;
    }
  }
};


function awsPlaySlower() {
  console.log("awsPlaySlower()");
  let vid = document.querySelector("video");
  if (vid) {
    if (vid.playbackRate > 0.25) {
      vid.playbackRate -= 0.25;
    }
  }
};

"#,
      )?;



      #[cfg(debug_assertions)]
      webview_window.open_devtools();

      webview_window.set_title("vidDeck")?;
        
      let play_pause_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyS,
      );

      let rewind_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyA,
      );

      let fast_forward_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyD,
      );

      let play_faster_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyC,
      );

      let play_slower_key = Shortcut::new(
        Some(Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyZ,
      );

      app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
          .with_handler(move |appx, shortcut, event| {
              if let ShortcutState::Pressed =
                event.state() {
                    if shortcut == &play_pause_key {
                        appx
                            .get_webview_window("main")
                            .expect("no main widow")
                            .eval(r#"awsPlayPause();"#)
                            .expect("eval did not work");
                    }
                    if shortcut == &rewind_key {
                        appx
                            .get_webview_window("main")
                            .expect("no main widow")
                            .eval(r#"awsRewind();"#)
                            .expect("eval did not work");
                    }
                    if shortcut == &fast_forward_key {
                        appx
                            .get_webview_window("main")
                            .expect("no main widow")
                            .eval(r#"awsFastForward();"#)
                            .expect("eval did not work");
                    }
                    if shortcut == &play_faster_key {
                        appx
                            .get_webview_window("main")
                            .expect("no main widow")
                            .eval(r#"awsPlayFaster();"#)
                            .expect("eval did not work");
                    }
                    if shortcut == &play_slower_key {
                        appx
                            .get_webview_window("main")
                            .expect("no main widow")
                            .eval(r#"awsPlaySlower();"#)
                            .expect("eval did not work");
                    }
                }
          })
          .build(),
      )?;
      app.global_shortcut().register(play_pause_key)?;
     app.global_shortcut().register(rewind_key)?;
     app.global_shortcut().register(fast_forward_key)?;
     app.global_shortcut().register(play_faster_key)?;
     app.global_shortcut().register(play_slower_key)?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//   format!(
//     "Hello, {}! You've been greeted from Rust!",
//     name
//   )
// }

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

// tauri::Builder::default()
//     .plugin(tauri_plugin_global_shortcut::Builder::new().build())
//     .plugin(tauri_plugin_opener::init())
//     .invoke_handler(tauri::generate_handler![greet])
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
