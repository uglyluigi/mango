const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function get_library() {
  return await invoke("get_library", {});
}

window.addEventListener("DOMContentLoaded", async () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  let libraryContainerEl = document.querySelector("#library-container");
  let library = await get_library();

  for (let series of library.series) {
    let imgEl = document.createElement("img");
    imgEl.src = series.covers[0].path;
    libraryContainerEl.appendChild(imgEl);
    console.log(imgEl.src);
  }
});
