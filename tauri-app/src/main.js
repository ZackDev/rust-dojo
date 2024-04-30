const { invoke } = window.__TAURI__.tauri;

/* name, identifier, stat */
let stats = [
  ["current date","current-date", "c"],
  ["first run", "first-run", "1"],
  ["last run", "last-run", "n"],
  ["total time", "total-time", "o"],
  ["average time", "average-time", "a"],
  ["min and max time", "min-max-time", "x"],
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

function setstats(forElement, statsStr) {
  let e = document.querySelector("#" + forElement + "-stats");
  e.innerText = statsStr;
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
    st.classList.add("stats-display");
    st.style.width = "auto";
    st.style.fontSize = "20px";

    let la = document.createElement("label");
    la.textContent = s[0];
    la.setAttribute("for", s[1] + "-cb");
    la.style.minWidth = "250px";
    la.style.textAlign = "end";
    
    let cb = document.createElement("input");
    cb.id = s[1] + "-cb";
    cb.type = "checkbox";
    cb.setAttribute("stats", "stats");
    cb.value = s[2];
    cb.style.width = "20px";
    cb.style.height = "20px";

    cb.addEventListener("click", (e) => {
      if (cb.checked) {
        getstats(cb.value).then(
          (v) => {
            setstats(s[1], v);
          },
          (f) => {

          }
        );
      }
      else {
        st.textContent = "";
      }
    });

    co.append(st, la, cb);
    statsControls.append(co);
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
    let cbs = document.querySelectorAll("input[stats]:checked");
    cbs.forEach((c) => {
      getstats(c.value).then(
        (v) => {
          for (let st in stats) {
            if (c.value == stats[st][2]) {
              setstats(stats[st][1], v);
              break;
            }
          }
        },
        (f) => {}
      );
    });
  });
});
