const { invoke } = window.__TAURI__.tauri;

class StatsObj {
  constructor(name, id, flag, uistyle) {
    this.name = name;
    this.id = id;
    this.flag = flag;
    this.uistyle = uistyle;
  }
  setValue(value) {
    this.value = value;
  }
}

const fontType = 'monospace';

const chartTitleCfg = {
  display: true,
  font: {
    family: fontType,
    size: 20,
  }
}

const chartTooltipCfg = {
  titleFont: {
    family: fontType
  },
  bodyFont: {
    family: fontType
  },
  footerFont: {
    family: fontType
  },
}

/* name, identifier, flag, style */
let stats = [
  ["current date", "current-date", "c", "text"],
  ["first run", "first-run", "1", "text"],
  ["last run", "last-run", "n", "text"],
  ["total time", "total-time", "o", "text"],
  ["average time", "average-time", "a", "text"],
  ["min and max time", "min-max-time", "x", "text"],
  ["number of rides", "num-rides", "r", "text"],
  ["minutes per day", "duration", "d", "chart"],
  ["rides per day", "frequency", "f", "chart"]
];

let chartsMap = new Map();

let timesInputEl;
let simpleStatsContainer;
let chartsContainer;

async function addtime(v) {
  await invoke("addtime", { time: v });
}

async function getstats(so) {
  let s = await invoke("getstats", { stat: so.flag });
  so.setValue(s);
}

function setstat(so) {
  let e = document.querySelector("#" + so.id + "-stats");
  if (so.uistyle == "text"){
    e.innerText = so.value;
  }
  else if (so.uistyle == "chart") {
    e.width = "auto";
    e.style.height = "200px";
    let json = JSON.parse(so.value);
    let cfgObj = {
      type: 'bar',
      data: {
        labels: json.labels,
        datasets: [{
          label: so.name,
          data: json.data,
          borderWidth: 1,
        }]
      },
      options: {
        plugins: {
          title: {
            text: so.name,
            display: true,
            font: chartTitleCfg.font,
          },
          tooltip: chartTooltipCfg,
          legend: {
            display: false,
          }
        },
        scales: {
          y: {
            beginAtZero: true
          }
        },
      }
    };

    if (chartsMap.has(so.id)) {
      chartsMap.get(so.id).destroy();
      chartsMap.delete(so.id);
    }
    let chart = new Chart(e, cfgObj);
    chartsMap.set(so.id, chart);
  }
}

function refreshStats(statsOb) {
  getstats(statsOb).then(
    (_) => {
      setstat(statsOb);
    },
    (f) => {

    }
  );
}

window.addEventListener("DOMContentLoaded", () => {
  simpleStatsContainer = document.querySelector("#stats-container");
  chartsContainer = document.querySelector("#charts-container");
  timesInputEl = document.querySelector("#times-input");

  stats.forEach((s) => {
    let so = new StatsObj(s[0], s[1], s[2], s[3]);
    if (so.uistyle == "text") {
      let st = document.createElement("div");
      st.id = so.id + "-stats";
      st.classList.add("stats-display");

      let co = document.createElement("div");
      co.classList.add("row", "stats-row");

      let la = document.createElement("label");
      la.style.fontSize = "20px";
      la.textContent = so.name + ":";
      la.setAttribute("for", so.id + "-cb");

      co.append(la, st);
      simpleStatsContainer.append(co);
    }
    else if (so.uistyle == "chart") {
      let st = document.createElement("div");
      st.class = "chart";

      let cv = document.createElement("canvas");
      cv.id = so.id + "-stats";

      st.append(cv);
      chartsContainer.append(st);
    }
    refreshStats(so);
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    let v = timesInputEl.value;
    let t = parseInt(v);
    if (t != NaN && t > 0) {
      addtime(v);
      stats.forEach((s) => {
        refreshStats(new StatsObj(s[0], s[1], s[2], s[3]));
      });
    }
  });
});
