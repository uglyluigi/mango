import {
  get_resource_server_url,
  get_all_titles,
  get_chapter_list,
} from "./invokes.js";
import {
  performStateTransition,
  chapterListState,
  libraryViewState,
  chapterViewState,
  is,
  toggleMenu,
} from "./state.js";

const libraryContainerEl = document.getElementById("library-container");
const chapterListEl = document.getElementById("chapter-list-container");
const chapterViewEl = document.getElementById("chapter-view-container");

// This field is set to the callback that's used to handle key events
// so it can be removed without being mistaken for a different keyboard
// callback that I may add later
let chapterViewArrowKeyAndEscapeKeyListener;
let chapterListEscapeKeyListener;

// Hides the elements in the UI document based on
// the stateTransition object.
// Does so by applying/removing a CSS class called "hidden"
function updateElementHiddenAttributes(stateTransition) {
  switch (stateTransition.to) {
    case libraryViewState:
      libraryContainerEl.classList.remove("hidden");
      break;
    case chapterListState:
      chapterListEl.classList.remove("hidden");
      break;
    case chapterViewState:
      chapterViewEl.classList.remove("hidden");
      break;
  }

  switch (stateTransition.from) {
    case libraryViewState:
      libraryContainerEl.classList.add("hidden");
      break;
    case chapterListState:
      chapterListEl.classList.add("hidden");
      break;
    case chapterViewState:
      chapterViewEl.classList.add("hidden");
      break;
  }
}

// Accepting covers from the back-end and
// creating their img elements on the UI
async function buildLibraryView() {
  buildMenu();
  document.addEventListener("keydown", ({ code } = e) => {
    if (code === "Space") toggleMenu();
  });
  let resource_server_url = await get_resource_server_url();

  let titles = await get_all_titles();

  for (let title of titles) {
    let coverContainer = document.createElement("div");
    coverContainer.classList.add("cover-container");
    coverContainer.classList.add("shrink-on-hover");

    // Title element
    let coverTitle = document.createElement("div");
    coverTitle.classList = "cover-title";
    coverTitle.textContent = title;

    // Image element that contains the cover data
    let imgEl = document.createElement("img");

    await fetch(`${resource_server_url}covers/${title}`).then(async (res) => {
      let blob = await res.blob();
      imgEl.classList.add("library-view-cover");
      imgEl.src = URL.createObjectURL(blob);

      const libraryCoverContainer = document.createElement("div");
      libraryCoverContainer.appendChild(imgEl);
      libraryCoverContainer.classList.add("library-view-cover-container");

      coverContainer.appendChild(libraryCoverContainer);
      coverContainer.appendChild(coverTitle);
      coverContainer.addEventListener("click", async () => {
        await performStateTransition(chapterListState, {
          title,
          imgSrc: imgEl.src,
        });
      });
      libraryContainerEl.appendChild(coverContainer);
    });
  }

  const icon = document.getElementById("mango-logo");
  icon.onclick = () => {
    toggleMenu();
  };
}

// Builds the chapter list
// Accepts the title of the series
// and the src URL of that series's cover
async function openChapterList({ title, imgSrc }) {
  let chapInfo = await get_chapter_list(title);

  makeBackButton(chapterListEl, async () => {
    await performStateTransition(libraryViewState);
  });

  chapInfo.sort((a, b) => {
    let [chapNum1, _] = a;
    let [chapNum2, __] = b;
    return chapNum1 > chapNum2;
  });

  const chapterTitleLabel = document.createElement("h2");
  chapterTitleLabel.id = "chapter-title-label";
  chapterTitleLabel.textContent = title;

  const actualChapterListEl = document.createElement("div");
  actualChapterListEl.id = "chapter-list";
  actualChapterListEl.appendChild(chapterTitleLabel);
  const bigCoverEl = document.createElement("img");
  bigCoverEl.id = "big-cover";
  chapterListEl.appendChild(actualChapterListEl);
  const bigCoverWrapper = document.createElement("div");
  bigCoverWrapper.id = "big-cover-wrapper";
  bigCoverWrapper.appendChild(bigCoverEl);
  chapterListEl.appendChild(bigCoverWrapper);

  for (let chapter of chapInfo) {
    let [chapNum, chapString] = chapter;

    let chapterEl = document.createElement("div");
    chapterEl.classList.add("chapter-list-entry");
    chapterEl.textContent = chapString;
    chapterEl.addEventListener("click", async () => {
      await performStateTransition(chapterViewState, {
        title,
        chapter: chapNum,
      });
    });
    actualChapterListEl.appendChild(chapterEl);
    bigCoverEl.src = imgSrc;
  }

  const keyboardCallback = async (e) => {
    // This callback can be active on the chapterView as well
    // in some situations due to not destroying the view before
    // it when moving to the chapterView.
    if (!is(chapterListState)) return;

    if (e.key === "Escape") {
      await performStateTransition(libraryViewState);
    }
  };

  document.addEventListener("keydown", keyboardCallback);
  chapterListEscapeKeyListener = keyboardCallback;
}

// Removes all children of the chapter-list container
function destroyChapterList() {
  chapterListEl.replaceChildren([]);
  document.removeEventListener("keydown", chapterListEscapeKeyListener);
}

async function openChapterView({ title, chapter }) {
  let resource_server_url = await get_resource_server_url();

  makeBackButton(chapterViewEl, async () => {
    // Does not require args, because the UI does not destroy
    // the chapterList for this chapter when moving to the chapterView
    await performStateTransition(chapterListState, null);
  });

  fetch(`${resource_server_url}image_count/${title}/${chapter}`).then(
    async (res) => {
      let numImages = parseInt(await res.text());
      let map = new Map();
      let promises = [];

      for (let i = 0; i < numImages; i++) {
        promises.push(
          fetch(
            `${resource_server_url}chapter_image/${title}/${chapter}/${i}`
          ).then(async (res) => {
            map.set(i, await res.blob());
          })
        );
      }

      Promise.all(promises).then(() => {
        let currentImg = 0;
        let imgEl = document.createElement("img");
        let imgWrapper = document.createElement("div");
        imgWrapper.classList.add("chapter-img-wrapper");
        imgWrapper.appendChild(imgEl);

        let updateImg = function () {
          let url = URL.createObjectURL(map.get(currentImg));
          imgEl.src = url;
          imgEl.onload = () => {
            URL.revokeObjectURL(url);
          };
        };

        let buttonL = document.createElement("button");
        buttonL.innerHTML = "PREV";
        let buttonR = document.createElement("button");
        buttonR.innerHTML = "NEXT";

        let updateDisbled = () => {
          const hidden = "visibility: hidden";
          buttonL.disabled = currentImg === 0;
          buttonL.style = buttonL.disabled ? hidden : "";
          buttonR.disabled = currentImg === numImages - 1;
          buttonR.style = buttonR.disabled ? hidden : "";
        };

        let moveL = () => {
          if (!buttonL.disabled) {
            currentImg--;
            updateImg();
            updateDisbled();
          }
        };

        let moveR = () => {
          if (!buttonR.disabled) {
            currentImg++;
            updateImg();
            updateDisbled();
          }
        };

        updateDisbled();
        updateImg();

        buttonL.addEventListener("click", () => {
          moveL();
        });

        buttonR.addEventListener("click", () => {
          moveR();
        });

        const keyboardCallback = async (e) => {
          switch (e.key) {
            case "ArrowRight":
              moveR();
              break;
            case "ArrowLeft":
              moveL();
              break;
            case "Escape":
              await performStateTransition(chapterListState, null);
              break;
            default:
              break;
          }
        };

        chapterViewEl.appendChild(buttonL);
        chapterViewEl.appendChild(imgWrapper);
        chapterViewEl.appendChild(buttonR);
        document.addEventListener("keydown", keyboardCallback);
        chapterViewArrowKeyAndEscapeKeyListener = keyboardCallback;
      });
    }
  );
}

async function destroyChapterView() {
  chapterViewEl.replaceChildren([]);
  document.removeEventListener(
    "keydown",
    chapterViewArrowKeyAndEscapeKeyListener
  );
}

function makeBackButton(parent, cb) {
  const button = document.createElement("label");
  button.onclick = cb;
  button.id = "back-button";
  button.classList.add("back-button");
  const buttonImg = document.createElement("img");
  buttonImg.src = "./assets/arrow_back.svg";
  button.classList.add("shrink-on-hover");
  button.appendChild(buttonImg);
  parent.appendChild(button);
}

function openMenu() {
  const bod = document.getElementById("scene-container");
  bod.classList.add("slide-over");
  let menu = document.getElementById("menu-container");
  menu.classList.remove("below");
}

function buildMenu() {
  const menu = document.getElementById("menu-container");
  const label = document.createElement("h1");

  label.textContent = "Menu";
  menu.appendChild(label);
  makeMenuEntry("Library", menu, async () => {
    if (!is(libraryViewState)) {
      await performStateTransition(libraryViewState);
    }
    closeMenu();
  });
  makeMenuEntry("Search", menu, () => {});
  makeMenuEntry("Bookmarks", menu, () => {});
}

function closeMenu() {
  const bod = document.getElementById("scene-container");
  bod.classList.remove("slide-over");
  const menu = document.getElementById("menu-container");
  menu.classList.add("below");
}

function makeMenuEntry(labelContent, parent, cb) {
  const entry = document.createElement("div");
  const label = document.createElement("label");
  label.textContent = labelContent;
  entry.classList.add("menu-entry");
  entry.appendChild(label);
  entry.onclick = cb;
  parent.appendChild(entry);
}

export {
  updateElementHiddenAttributes,
  buildLibraryView,
  openChapterList,
  destroyChapterList,
  openChapterView,
  destroyChapterView,
  openMenu,
  closeMenu,
};
