const { invoke } = window.__TAURI__.tauri;

let statsBoxes;
let timesInputEl;
let msgEl;

async function addtime() {
  msgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

async function getstats(statsStr) {
  msgEl.textContent = await invoke("getstats", { options: statsStr});
}

function getStatsBoxes() {
  let boxes = document.querySelectorAll("input[stats]");
  return boxes;
}

function getCheckedStatsBoxes() {
  let boxes = document.querySelectorAll("input[stats]:checked");
  return boxes;
}

window.addEventListener("DOMContentLoaded", () => {
  timesInputEl = document.querySelector("#times-input");
  msgEl = document.querySelector("#message-container"); 
  statsBoxes = getStatsBoxes();

  statsBoxes.forEach((sb) => {
    sb.addEventListener("click", (e) => {
      let statsOptStr = "";
      let checkedStatsBoxes = getCheckedStatsBoxes();
      checkedStatsBoxes.forEach((cb) => {
        statsOptStr += cb.value;
      });
      msgEl.textContent = getstats(statsOptStr);
    });
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
