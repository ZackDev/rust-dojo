const { invoke } = window.__TAURI__.tauri;

let timesInputEl;
let timesMsgEl;

/*
stats checkboxes
*/
let cCb;
let OneCb;
let lCb;
let tCb;
let aCb;
let xCb;
let nCb;
let fCb;
let dCb;

let cbs = [cCb, OneCb, lCb, tCb, aCb, xCb, nCb, fCb, dCb];

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  /* TODO add checkbox references */
  cCb = document.querySelector("current-date-cb");
  OneCb = document.querySelector("first-run-cb");
  lCb = document.querySelector("last-run-cb");
  tCb = document.querySelector("total-time-cb");
  aCb = document.querySelector("average-time-cb");
  xCb = document.querySelector("min-max-time-cb");
  nC = document.querySelector("num-rides-cb");
  fCb = document.querySelector("frequency-cb");
  dCb = document.querySelector("duration-cb");
  
  /* TODO call 'stats' backend when ANY of the checkboxes changes and pass every checked stats */
  cbs.forEach(cb => {
    cb.addEventListener("onchange", (e) => {
      e.preventDefault();
      // TODO build the stats string and pass it to the backend
    })
  })

  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
