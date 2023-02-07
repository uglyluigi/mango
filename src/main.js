import { initState, render } from "./state.js";

window.addEventListener("DOMContentLoaded", async () => {
  initState();
  await render();
});
