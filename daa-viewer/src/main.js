const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

 function query() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // greetMsgEl.textContent = await invoke("query", { name: greetInputEl.value });

  invoke('query', { sql: greetInputEl.value  })
      .then(data => {
        const container = document.querySelector("#container");
        container.innerHTML = `<pre>${data}</pre>`;
      })
      .catch(e => alert(e));
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    query();
  });
});
