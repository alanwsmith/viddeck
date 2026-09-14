const { invoke } = window.__TAURI__.core;
const { register } = window.__TAURI__.globalShortcut;

await register("CommandOrControl+Shift+C", () => {
  console.log("Shortcut triggered");
});

/*
window.addEventListener("DOMContentLoaded", () => {
});
*/
