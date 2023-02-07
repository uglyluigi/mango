import { get_resource_server_url, get_library } from "./invokes.js";

const libraryViewState = Symbol("libraryView");
const chapterListState = Symbol("chapterList");
const chapterViewState = Symbol("chapterView");
const dialogView = Symbol("dialogView");

const libraryContainerEl = document.getElementById("library-container");
const chapterListEl = document.getElementById("chapter-list-container");
const chapterViewEl = document.getElementById("chapter-view-container");

let currentStateValue;

function initState() {
  currentStateValue = {
    currentStateSymbol: libraryViewState,
    libraryViewValid: false,
    hooks: [],
  };
}

function currentState() {
  return currentStateValue;
}

// Register a state hook that runs when
// the state transitions from the "from"
// symbol to the "to" symbol
function registerStateHook(f, from, to, oneshot = false) {
  currentState.hooks.push({
    callback: f,
    stateTransition: {
      from,
      to,
    },
    oneshot,
  });
}

function registerOneshotHook(f, from, to) {
  registerStateHook(f, from, to, true);
}

// Based on current state values, produce
// the desired UI
async function render() {
  let state = currentState();

  switch (state.currentStateSymbol) {
    case libraryViewState:
      if (!state.libraryViewValid) {
        await buildLibraryView();
        state.libraryViewValid = true;
      }
      break;
    case chapterListState:
      break;
    case chapterViewState:
      break;
  }
}

// Accepting covers from the back-end and
// creating their img elements on the UI
async function buildLibraryView() {
  console.log("library view");

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
      coverContainer.addEventListener("click", async () => {
        await performStateTransition(chapterListState);
      });
      libraryContainerEl.appendChild(coverContainer);
    });
  }
}

async function performStateTransition(newStateSymbol) {
  if (newStateSymbol !== currentStateValue.currentStateSymbol) {
    console.log("1");

    const stateTransition = {
      from: currentStateValue.currentStateSymbol,
      to: newStateSymbol,
    };

    let newHooks = [];

    for (hook of currentStateValue.hooks) {
      if (hook.stateTransition === stateTransition) {
        hook.callback();

        if (!hook.oneshot) {
          newHooks.push(hook);
        }
      } else {
        newHooks.push(hook);
      }
    }

    currentStateValue.hooks = newHooks;
    currentStateValue.currentStateSymbol = newStateSymbol;

    // Hide stuff that shouldn't be showing; unhide the other stuff!
    let currentSymbol = currentStateValue.currentStateSymbol;

    libraryContainerEl.classList.add("hidden");

    chapterListEl.hidden = currentSymbol !== chapterListState;
    chapterViewEl.hidden = currentSymbol !== chapterViewState;

    switch (stateTransition.to) {
      case libraryViewState:
        break;
      case chapterListState:
        break;
      case chapterViewState:
        break;
    }

    await render();
  } else {
    console.log(
      `Erroneous state transition occurred (current state is ${currentState})`
    );
  }
}

async function requestCoverForSeries(seriesTitle, cb) {
  let resource_server_url = await get_resource_server_url();
  let url = new URL(`${resource_server_url}covers/${seriesTitle}`);
  let xmlHttp = new XMLHttpRequest();
  xmlHttp.open("GET", url, true);
  xmlHttp.send();

  xmlHttp.onload = function () {
    if (xmlHttp.readyState === 4) {
      if (xmlHttp.status === 200) {
        cb(xmlHttp.responseText);
      }
    }
  };
}

async function invalidateLibraryView() {
  currentStateValue.libraryViewValid = false;
  await render();
}

async function openChapterView() {}

export { initState, render, performStateTransition };
