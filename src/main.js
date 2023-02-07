import { initState, render } from "./state.js";
import { get_chapter_list } from "./invokes.js";

window.addEventListener("DOMContentLoaded", async () => {
  initState();
  await render();
});
