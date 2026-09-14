import { Window } from "@tauri-apps/api/window"
import { Webview } from "@tauri-apps/api/webview"

const appWindow = new Window('uniqueLabel');

appWindow.once('tauri://created', async function () {
  // `new Webview` Should be called after the window is successfully created,
  // or webview may not be attached to the window since window is not created yet.

  // // loading embedded asset:
  // const webview = new Webview(appWindow, 'theUniqueLabel', {
  //   url: 'path/to/page.html',
  //   // create a webview with specific logical position and size
  //   x: 0,
  //   y: 0,
  //   width: 800,
  //   height: 600,
  // });

  // alternatively, load a remote URL:
  const webview = new Webview(appWindow, 'theUniqueLabel', {
    url: 'https://github.com/tauri-apps/tauri',

    // create a webview with specific logical position and size
    x: 0,
    y: 0,
    width: 800,
    height: 600,
  });

  webview.once('tauri://created', function () {
    // webview successfully created
  });
  webview.once('tauri://error', function (e) {
    // an error happened creating the webview
  });

  // emit an event to the backend
  await webview.emit("some-event", "data");
  // listen to an event from the backend
  const unlisten = await webview.listen("event-name", e => { });
  unlisten();
});


// const { invoke } = window.__TAURI__.core;
// const { register } = window.__TAURI__.globalShortcut;

// await register("CommandOrControl+Shift+C", () => {
//     //console.log("Shortcut triggered");
//     document.querySelector("#videoFrame").contentWindow.postMesage(
// 	{
// 	    type: "playpause", data: {}
// 	},
// 	'al9000.com'
//     );
//     document.querySelector("#status").innerHTML = new Date();
// });

/*
window.addEventListener("DOMContentLoaded", () => {
});
*/
