const { invoke } = window.__TAURI__.tauri;

let timesInputEl;
let timesMsgEl;

/*
stats checkboxes
*/
let currentDateCb;
let firstRunCb;
let lastRunCb;
let totalTimeCb;
let averageTimeCb;
let minAndMaxTimeCb;
let numRidesCb;
let frequencyCb;
let durationCb;

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");
  
  /* TODO add checkbox references */

  /* TODO call 'stats' backend when ANY of the checkboxes changes and pass every checked stats */

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
