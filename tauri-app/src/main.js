const { invoke } = window.__TAURI__.tauri;

/* name, identifier, stat short */
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
let durChart;
let freqChart;

let timesInputEl;
let msgEl;

async function addtime() {
  msgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

async function getstats(statsChr) {
  let s = await invoke("getstats", { stat: statsChr });
  return s;
}

function setstat(forElementIDStr, statsStr, stat) {
  let e = document.querySelector("#" + forElementIDStr + "-stats");
  let value = statsStr;
  if (stat == "f") {
    if (freqChart != undefined || freqChart != null) {
      freqChart.destroy();
    }
    let json = JSON.parse(statsStr);;
    e.width = "auto";
    e.style.height = "400px";
    freqChart = new Chart(e, {
      type: 'bar',
      data: {
        labels: json.dates,
        datasets: [{
          label: 'frequency',
          data: json.frequency,
          borderWidth: 1
        }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true
          }
        },
      }
    });
  }
  if (stat == "d") {
    value = statsStr.duration;
  }
  else {
    e.innerText = value;
  }
}

function getStatsBoxes() {
  return document.querySelectorAll("input[stats]");
}

function getCheckedStatsBoxes() { 
  return document.querySelectorAll("input[stats]:checked");
}

window.addEventListener("DOMContentLoaded", () => {
  statsControls = document.querySelector("#stats-container");
  timesInputEl = document.querySelector("#times-input");
  msgEl = document.querySelector("#message-container");
  
  stats.forEach((s) => {
    let st;
    if (s[2] == "f" || s[2] == "d") {
      getstats(s[2]).then(
        (v) => {
          setstat(s[1], v, s[2]);
        },
        (f) => {
  
        }
      );

    } else {
      st = document.createElement("div");
      st.id = s[1] + "-stats";
      st.classList.add("stats-display");
      st.style.width = "auto";
      st.style.fontSize = "20px";
      
      let co = document.createElement("div");
      co.classList.add("row", "stats-row");
  
      let la = document.createElement("label");
      la.textContent = s[0];
      la.setAttribute("for", s[1] + "-cb");
      la.style.minWidth = "250px";
      la.style.textAlign = "end";
      
      getstats(s[2]).then(
        (v) => {
          setstat(s[1], v, s[2]);
        },
        (f) => {
  
        }
      );
  
      co.append(la, st);
      statsControls.append(co);
    }
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
    stats.forEach((s) => {
      getstats(s[2]).then(
        (v) => {
          setstat(s[1], v, s[2]);
        },
        (f) => {
  
        }
      );
    });
  });
});
