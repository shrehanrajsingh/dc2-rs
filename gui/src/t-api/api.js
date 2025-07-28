import { invoke } from "@tauri-apps/api/core";

export const check_server_is_running = async () => {
  return await invoke("server_is_running", {});
};

export const get_peers = async () => {
  return await invoke("get_peers", {});
};
