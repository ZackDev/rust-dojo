const { invoke } = window.__TAURI__.tauri;

let cbs;

let timesInputEl;
let timesMsgEl;

/*
stats checkboxes
*/
let c;
let o;
let l;
let t;
let a;
let x;
let n;
let f;
let d;

async function addtime() {
  timesMsgEl.textContent = await invoke("addtime", { time: timesInputEl.value });
}

function getCheckedBoxes() {
  let statStr = "";
  if (c.checked) {
    statStr += "c";
  }
  if (o.checked) {
    statStr += "1";
  }
  if (l.checked) {
    statStr += "l";
  }
  if (t.checked) {
    statStr += "t";
  }
  if (a.checked) {
    statStr += "a";
  }
  if (x.checked) {
    statStr += "x";
  }
  if (n.checked) {
    statStr += "n";
  }
  if (f.checked) {
    statStr += "f";
  }
  if (d.checked) {
    statStr += "d";
  }
  return statStr;
}

window.addEventListener("DOMContentLoaded", () => {
  timesInputEl = document.querySelector("#times-input");
  timesMsgEl = document.querySelector("#times-container");

  /* TODO add checkbox references */
  
  c = document.querySelector("#current-date-cb");
  o = document.querySelector("#first-run-cb");
  l = document.querySelector("#last-run-cb");
  t = document.querySelector("#total-time-cb");
  a = document.querySelector("#average-time-cb");
  x = document.querySelector("#min-max-time-cb");
  n = document.querySelector("#num-rides-cb");
  f = document.querySelector("#frequency-cb");
  d = document.querySelector("#duration-cb");
  
  cbs = [c, o, l, t, a, x, n, f, d];

  /* TODO call 'stats' backend when ANY of the checkboxes changes and pass every checked stats */
  cbs.forEach((cb) => {
    cb.addEventListener("click", (e) => {
      // timesMsgEl.textContent = getCheckedBoxes();
      let statStr = getCheckedBoxes();
    });
  });

  document.querySelector("#times-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addtime();
  });
});
