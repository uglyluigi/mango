const { invoke } = window.__TAURI__.tauri;

async function get_library() {
  return await invoke("get_library", {});
}

// Shouldn't take long :^)
async function get_resource_server_url() {
  return await invoke("get_resource_server_url", {});
}

export { get_library, get_resource_server_url };
