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

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  /* TODO add checkbox references */
  cCb = document.querySelector();
  OneCb = document.querySelector();
  lCb = document.querySelector();
  tCb = document.querySelector();
  aCb = document.querySelector();
  xCb = document.querySelector();
  nC = document.querySelector();
  fCb = document.querySelector();
  dCb = document.querySelector();
  
  /* TODO call 'stats' backend when ANY of the checkboxes changes and pass every checked stats */
  
  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");
  


  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
