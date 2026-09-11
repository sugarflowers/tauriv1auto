const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
  const msg = await invoke("greet", { name: greetInputEl.value });
  greetMsgEl.textContent = msg;
  alert(msg);
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  if (window.__TAURI__) {
    greetMsgEl.textContent = "Hello, Tauri!";
    const appWindow = window.__TAURI__.window.appWindow;

    appWindow.listen("tauri://resize", ({ payload }) => {
      const {width, height} = payload;
      greetMsgEl.textContent = `${width}, ${height}`;
    });
  }
});
