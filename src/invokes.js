const { invoke } = window.__TAURI__.tauri;

// Returns the URL of the warp server responsible
// for serving image blobs to the front-end
async function get_resource_server_url() {
  return await invoke("get_resource_server_url", {});
}

// Returns a list of tuples containing the chapter's number
// at 0 and a formatted string following the format "Chapter [chapter_num]"
async function get_chapter_list(series) {
  return await invoke("get_chapter_list", { series });
}

// Returns an array containing the titles of all series in the
// library
async function get_all_titles() {
  return await invoke("get_all_titles", {});
}

export {
  get_resource_server_url,
  get_chapter_list,
  get_all_titles
};
