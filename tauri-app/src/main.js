const { invoke } = window.__TAURI__.tauri;

class StatsObj {
  constructor(name, id, flag) {
    this.name = name;
    this.id = id;
    this.flag = flag;
  }
  setValue(value) {
    this.value = value;
  }
}

const fontType = 'monospace';

const titlePlugin = {
  display: true,
  font: {
    family: fontType,
    size: 20,
  }
}

const tooltipPlugin = {
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

/* name, identifier, stat short */
let stats = [
  ["current date", "current-date", "c"],
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

async function addtime(v) {
  await invoke("addtime", { time: v });
}

async function getstats(so) {
  let s = await invoke("getstats", { stat: so.flag });
  so.setValue(s);
}

function setstat(so) {
  if (so.flag == "d" || so.flag == "f") {
    let e = document.querySelector("#" + so.id + "-stats");
    e.width = "auto";
    e.style.height = "400px";
    let json = JSON.parse(so.value);
    let cfgObj = {
      type: 'bar',
      data: {
        labels: json.dates,
      },
      options: {
        plugins: {
          title: {
            display: true,
            font: titlePlugin.font,
          },
          tooltip: tooltipPlugin,
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
    if (so.flag == "d") {
      if (durChart != undefined || durChart != null) {
        durChart.destroy();
      }
      cfgObj.data.datasets = [
        {
          label: 'duration',
          data: json.duration,
          borderWidth: 1
        }
      ];
      cfgObj.options.plugins.title.text = 'minutes per day';
      durChart = new Chart(e, cfgObj);
    }
    else if (so.flag == "f") {
      if (freqChart != undefined || freqChart != null) {
        freqChart.destroy();
      }
      cfgObj.data.datasets = [
        {
          label: 'frequency',
          data: json.frequency,
          borderWidth: 1
        }
      ];
      cfgObj.options.plugins.title.text = 'rides per day';
      freqChart = new Chart(e, cfgObj);
    }
  }
  else {
    let e = document.querySelector("#" + so.id + "-stats");
    e.innerText = so.value;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  statsControls = document.querySelector("#stats-container");
  timesInputEl = document.querySelector("#times-input");

  stats.forEach((s) => {
    let so = new StatsObj(s[0], s[1], s[2]);
    let st;
    if (so.flag == "f" || so.flag == "d") {
      getstats(so).then(
        (_) => {
          setstat(so);
        },
        (f) => {

        }
      );

    } else {
      st = document.createElement("div");
      st.id = so.id + "-stats";
      st.classList.add("stats-display");

      let co = document.createElement("div");
      co.classList.add("row", "stats-row");

      let la = document.createElement("label");
      la.style.fontSize = "20px";
      la.textContent = so.name + ":";
      la.setAttribute("for", so.id + "-cb");

      getstats(so).then(
        (_) => {
          setstat(so);
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
    let v = timesInputEl.value;
    let t = parseInt(v);
    if (t != NaN && t > 0) {
      addtime(v);
      stats.forEach((s) => {
        let so = new StatsObj(s[0], s[1], s[2]);
        getstats(so).then(
          (_) => {
            setstat(so);
          },
          (f) => {

          }
        );
      });
    }
  });
});
