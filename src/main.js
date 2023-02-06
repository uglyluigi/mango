const { invoke } = window.__TAURI__.tauri;

async function get_library() {
  return await invoke("get_library", {});
}

// Shouldn't take long :^)
async function get_resource_server_url() {
  return await invoke("get_resource_server_url", {});
}

window.addEventListener("DOMContentLoaded", async () => {
  let libraryContainerEl = document.querySelector("#library-container");
  let library = await get_library();
  let resource_server_url = await get_resource_server_url();

  for (let series of library.series) {
    let imgEl = document.createElement("img");
    let url = new URL(`${resource_server_url}covers/${series.title}`);
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", url, true);
    xmlHttp.send(null);

    xmlHttp.onreadystatechange = function () {
      libraryContainerEl.appendChild(imgEl);
      //FIXME this is truly horrible!!!!
      // Right now it's sending over arrays of HUGE base64 blobs
      // and then parsing them... ouch
      // maybe just return the currently selected cover instead?
      // will probably just redo this whole system
      imgEl.src = JSON.parse(xmlHttp.responseText)[0];
      imgEl.height = 150;
      imgEl.width = imgEl.height / 1.5;
    };
  }
});
