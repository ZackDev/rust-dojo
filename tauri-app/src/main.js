const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let timesInputEl;
let timesMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function time() {
  timesMsgEl.textContent = await invoke("time", { time: timesInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");
  
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    time();
  });
});
