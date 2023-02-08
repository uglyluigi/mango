import {
  updateElementHiddenAttributes,
  buildLibraryView,
  openChapterList,
  closeChapterList,
} from "./ui.js";

const initializationState = Symbol("init");
const libraryViewState = Symbol("libraryView");
const chapterListState = Symbol("chapterList");
const chapterViewState = Symbol("chapterView");
const dialogView = Symbol("dialogView");

let currentStateValue;

function initState() {
  currentStateValue = {
    currentStateSymbol: initializationState,
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
async function render(stateTransition, args) {
  let { from, to } = stateTransition;

  updateElementHiddenAttributes(stateTransition);

  switch (from) {
    case libraryViewState:
      break;
    case chapterListState:
      closeChapterList();
      break;
    case chapterViewState:
      break;
  }

  switch (to) {
    case libraryViewState:
      if (!currentStateValue.libraryViewValid) {
        await buildLibraryView();
        currentStateValue.libraryViewValid = true;
      }
      break;
    case chapterListState:
      await openChapterList(args);
      break;
    case chapterViewState:
      break;
  }
}

// For chapterListState, args look like this:
// { title: String, imgSrc: String }
async function performStateTransition(newStateSymbol, args) {
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

    currentStateValue.hooks = newHooks;
    currentStateValue.currentStateSymbol = newStateSymbol;

    // Hide stuff that shouldn't be showing; unhide the other stuff!
    await render(stateTransition, args);
  } else {
    console.log(
      `Erroneous state transition occurred (current state is ${currentState})`
    );
  }
}

function invalidateLibraryView() {
  currentStateValue.libraryViewValid = false;
}

export {
  initState,
  performStateTransition,
  chapterListState,
  chapterViewState,
  libraryViewState,
};
