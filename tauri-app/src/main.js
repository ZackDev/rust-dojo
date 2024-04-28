const { invoke } = window.__TAURI__.tauri;

/* name, identifier, stat */
let stats = [
  ["current date","current-date", "c"],
  ["first run", "first-run", "1"],
  ["last run", "last-run", "n"],
  ["total time", "total-time", "t"],
  ["average time", "average-time", "a"],
  ["number of rides", "num-rides", "r"],
  ["duration", "duration", "d"],
  ["frequency", "frequency", "f"]
];

let statsControls;
let timesInputEl;
let msgEl;

async function addtime() {
  msgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

async function getstats(statsChr) {
  let s = await invoke("getstats", { stat: statsChr});
  return s;
}

function getStatsBoxes() {
  return document.querySelectorAll("input[stats]");
}

function getCheckedStatsBoxes() { 
  return document.querySelectorAll("input[stats]:checked");
}

window.addEventListener("DOMContentLoaded", () => {
  statsControls = document.querySelector("#stats-controls");
  timesInputEl = document.querySelector("#times-input");
  msgEl = document.querySelector("#message-container");
  
  stats.forEach((s) => {
    let co = document.createElement("div");
    co.classList.add("row", "stats-row");

    let st = document.createElement("div");
    st.id = s[1] + "-stats";

    let la = document.createElement("label");
    la.textContent = s[0];
    la.setAttribute("for", s[1] + "-cb");
    la.style = "min-width:150px;";
    
    let cb = document.createElement("input");
    cb.id = s[1] + "-cb";
    cb.type = "checkbox";
    cb.setAttribute("stats", "stats");
    cb.value = s[2];

    cb.addEventListener("click", (e) => {
      if (cb.checked) {
        st.innerText = getstats(cb.value);
      }
      else {
        st.textContent = "";
      }
    });

    co.append(st, la, cb);
    statsControls.append(co);
  });

  /*
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
  */
  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
