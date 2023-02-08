const { invoke } = window.__TAURI__.tauri;

async function get_library() {
  return await invoke("get_library", {});
}

// Shouldn't take long :^)
async function get_resource_server_url() {
  return await invoke("get_resource_server_url", {});
}

async function get_chapter_list(series) {
  return await invoke("get_chapter_list", { series });
}

async function get_chapter_list_2(series) {
  return await invoke("get_chapter_list_2", { series });
}

export {
  get_library,
  get_resource_server_url,
  get_chapter_list,
  get_chapter_list_2,
};
