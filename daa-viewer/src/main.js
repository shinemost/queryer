const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

 function query() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // greetMsgEl.textContent = await invoke("query", { name: greetInputEl.value });
     const container = document.querySelector("#container");
  invoke('query', { sql: greetInputEl.value  })
      .then(data => {
        container.innerHTML = `<pre>${data}</pre>`;
      })
      .catch(e => {
          container.innerHTML = `<pre>${e}</pre>`;
      });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    query();
  });
});
