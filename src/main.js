import { initState, libraryViewState, performStateTransition } from "./state.js";

window.addEventListener("DOMContentLoaded", async () => {
  initState();
  await performStateTransition(libraryViewState);
});
