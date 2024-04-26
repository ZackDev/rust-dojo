const { invoke } = window.__TAURI__.tauri;

let statsBoxes;

let timesInputEl;
let timesMsgEl;

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
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
  timesMsgEl = document.querySelector("#times-container");

  /* TODO add checkbox references */
  
  statsBoxes = getStatsBoxes();

  /* TODO call 'stats' backend when ANY of the checkboxes changes and pass every checked stats */
  statsBoxes.forEach((sb) => {
    sb.addEventListener("click", (e) => {
      let statsStr = "";
      let checkedStatsBoxes = getCheckedStatsBoxes();
      checkedStatsBoxes.forEach((b) => {
        statsStr += b.value;
      });
      timesMsgEl.textContent = statsStr;
    });
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
