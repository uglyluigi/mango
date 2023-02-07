const { invoke } = window.__TAURI__.tauri;

async function get_library() {
  return await invoke("get_library", {});
}

// Shouldn't take long :^)
async function get_resource_server_url() {
  return await invoke("get_resource_server_url", {});
}

window.addEventListener("DOMContentLoaded", async () => {
  await buildLibraryView();
});

async function buildLibraryView() {
  let libraryContainerEl = document.querySelector("#library-container");
  let library = await get_library();

  for (let series of library.series) {
    let coverContainer = document.createElement("div");
    coverContainer.classList.add("cover-container");
    coverContainer.classList.add("shrink-on-hover");

    // Title element
    let coverTitle = document.createElement("h3");
    coverTitle.classList = "cover-title";
    coverTitle.textContent = series.title;

    // Image element that contains the cover data
    let imgEl = document.createElement("img");

    await requestCoverForSeries(series.title, function (responseText) {
       //FIXME this is truly horrible!!!!
      // Right now it's sending over arrays of HUGE base64 blobs
      // and then parsing them... ouch
      // maybe just return the currently selected cover instead?
      // will probably just redo this whole system
      imgEl.src = JSON.parse(responseText)[0];
      imgEl.height = 150;
      imgEl.width = imgEl.height / 1.5;
      coverContainer.appendChild(imgEl);
      coverContainer.appendChild(coverTitle);
      coverContainer.addEventListener("click", () => {
        
      });
      libraryContainerEl.appendChild(coverContainer);
    });
  }
}

async function requestCoverForSeries(seriesTitle, cb) {
  let resource_server_url = await get_resource_server_url();
  let url = new URL(`${resource_server_url}covers/${seriesTitle}`);
  let xmlHttp = new XMLHttpRequest();
  xmlHttp.open("GET", url, true);
  xmlHttp.send();

  xmlHttp.onload = function() {
    if (xmlHttp.readyState === 4) {
      if (xmlHttp.status === 200) {
        cb(xmlHttp.responseText);
      }
    }
  };
}

async function openChapterView() {

}
