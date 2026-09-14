const { invoke } = window.__TAURI__.core;
const { register } = window.__TAURI__.globalShortcut;

await register("CommandOrControl+Shift+C", () => {
    console.log("Shortcut triggered");
    document.querySelector("#status").innerHTML = new Date();
});

/*
window.addEventListener("DOMContentLoaded", () => {
});
*/
