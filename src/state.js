import { updateElementHiddenAttributes, buildLibraryView } from "./ui.js";

const libraryViewState = Symbol("libraryView");
const chapterListState = Symbol("chapterList");
const chapterViewState = Symbol("chapterView");
const dialogView = Symbol("dialogView");

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

async function performStateTransition(newStateSymbol) {
  if (newStateSymbol !== currentStateValue.currentStateSymbol) {
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

    updateElementHiddenAttributes(stateTransition);
    currentStateValue.hooks = newHooks;
    currentStateValue.currentStateSymbol = newStateSymbol;

    // Hide stuff that shouldn't be showing; unhide the other stuff!
    await render();
  } else {
    console.log(
      `Erroneous state transition occurred (current state is ${currentState})`
    );
  }
}

async function invalidateLibraryView() {
  currentStateValue.libraryViewValid = false;
  await render();
}

async function openChapterView(series, chapter) {
	
}

export {
  initState,
  render,
  performStateTransition,
  chapterListState,
  chapterViewState,
  libraryViewState,
};
