import {
  updateElementHiddenAttributes,
  buildLibraryView,
  openChapterList,
  destroyChapterList,
  openChapterView,
  destroyChapterView,
} from "./ui.js";

// The state that the currentStateValue object defaults to
// on creation
const initializationState = Symbol("init");
// The state that is active when the user is on the libraryView
const libraryViewState = Symbol("libraryView");
// The state that is active when the user has clicked on a series
// in the library view and is viewing its list of chapters
const chapterListState = Symbol("chapterList");
// The state that is active when the user has clicked on a chapter
// in a series and is viewing the chapter's images
const chapterViewState = Symbol("chapterView");
// The state that is active when there is a dialog window open
const dialogView = Symbol("dialogView");

let currentStateValue;

// Initializes the default state object. Should only be called once
// Sets the current state to the initializationState, which doesn't
// render anything on the UI, but might in the future.
// main.js immediately moves to the libraryViewState anyway.
// Also holds the value that tracks whether the library view needs to be
// re-built. It is checked when moving between states.
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

// Based on current state values, call the ui
// functions to update the UI to match the state
// Also hides other components from inactive states
async function render(stateTransition, args) {
  let { from, to } = stateTransition;

  updateElementHiddenAttributes(stateTransition);

  switch (from) {
    case libraryViewState:
      break;
    case chapterListState:
      if (to !== chapterViewState) {
        destroyChapterList();
      }
      break;
    case chapterViewState:
      destroyChapterView();
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
      if (from !== chapterViewState) {
        await openChapterList(args);
      }
      break;
    case chapterViewState:
      await openChapterView(args);
      break;
  }
}

// This function is the only method that other parts of the front-end
// can use to change the state of the front-end.
// This function accepts a stateTransition object that looks like this:
// { from: (state symbol), to: (state symbol) }
// As well as an optional args parameter that can contain arbitrary data
// that is useful to expose to whatever functions will be running when
// you are moving between states. For example:
// for chapterListState, args look like this:
// { title: String, imgSrc: String }
// This object contains the title of the series that was clicked on
// in addition to a string containing the URL of that series's cover.
// Both of these pieces of information are displayed on the front-end
// when entering the chapterList state, so it makes sense to provide them
// when you are requesting a state transition.
// State transitions that do not change the current state symbol are not
// performed and are considered erroneous.
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
