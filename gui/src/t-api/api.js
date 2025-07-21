import { invoke } from "@tauri-apps/api/core";

export const check_server_is_running = async () => {
  return await invoke("server_is_running", {});
};
