const { invoke } = window.__TAURI__.tauri;

let timesInputEl;
let timesMsgEl;

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time_str: timesInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");
  
  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
