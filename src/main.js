import { initState, libraryViewState, performStateTransition } from "./state.js";
import { get_chapter_images } from "./invokes.js";

window.addEventListener("DOMContentLoaded", async () => {
  initState();
  await performStateTransition(libraryViewState);
});
