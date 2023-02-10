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
} from "./state.js";

const libraryContainerEl = document.getElementById("library-container");
const chapterListEl = document.getElementById("chapter-list-container");
const chapterViewEl = document.getElementById("chapter-view-container");

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
      //chapterViewState.classList.remove("hidden");
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
      //chapterViewState.classList.add("hidden");
      break;
  }
}

// Accepting covers from the back-end and
// creating their img elements on the UI
async function buildLibraryView() {
  let resource_server_url = await get_resource_server_url();

  let titles = await get_all_titles();

  for (let title of titles) {
    let coverContainer = document.createElement("div");
    coverContainer.classList.add("cover-container");
    coverContainer.classList.add("shrink-on-hover");

    // Title element
    let coverTitle = document.createElement("span");
    coverTitle.classList = "cover-title";
    coverTitle.textContent = title;

    // Image element that contains the cover data
    let imgEl = document.createElement("img");

    await fetch(`${resource_server_url}covers/${title}`).then(async (res) => {
      let blob = await res.blob();
      imgEl.src = URL.createObjectURL(blob);
      imgEl.height = 150;
      imgEl.width = imgEl.height / 1.5;

      coverContainer.appendChild(imgEl);
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
}

// Builds the chapter list
// Accepts the title of the series
// and the src URL of that series's cover
async function openChapterList({ title, imgSrc }) {
  let chapInfo = await get_chapter_list(title);

  makeBackButton(chapterListEl, async function () {
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
  chapterListEl.appendChild(chapterTitleLabel);

  const actualChapterListEl = document.createElement("div");
  actualChapterListEl.id = "chapter-list";
  const bigCoverEl = document.createElement("img");
  bigCoverEl.id = "big-cover";
  chapterListEl.appendChild(actualChapterListEl);
  chapterListEl.appendChild(bigCoverEl);

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
    // Looks great!
    bigCoverEl.height = 500;
    bigCoverEl.width = bigCoverEl.height / 1.5;
  }
}

// Removes all children of the chapter-list container
function destroyChapterList() {
  chapterListEl.replaceChildren([]);
}

async function openChapterView({ title, chapter }) {
  makeBackButton(chapterViewEl, async function () {
    // Does not require args, because the UI does not destroy
    // the chapterList for this chapter when moving to the chapterView
    await performStateTransition(chapterListState, null);
  });
}

async function destroyChapterView() {
  chapterViewEl.replaceChildren([]);
}

function makeBackButton(parent, cb) {
  const button = document.createElement("label");
  button.onclick = cb;
  button.id = "chapter-list-close-button";
  button.classList.add("back-button");
  const buttonImg = document.createElement("img");
  buttonImg.src = "./assets/arrow_back.svg";
  button.classList.add("shrink-on-hover");
  button.appendChild(buttonImg);
  parent.appendChild(button);
}

export {
  updateElementHiddenAttributes,
  buildLibraryView,
  openChapterList,
  destroyChapterList,
  openChapterView,
  destroyChapterView,
};
