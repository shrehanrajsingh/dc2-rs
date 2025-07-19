import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Footer() {
  const check_server_is_running = async () => {
    return await invoke("server_is_running", {});
  };

  const [status, setStatus] = useState(false);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        const isRunning = await check_server_is_running();
        setStatus(isRunning);
      } catch (error) {
        console.error("Failed to check server status:", error);
        setStatus(false);
      }
    };

    checkStatus();
    const intervalId = setInterval(checkStatus, 3000);
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="fixed flex bottom-0 w-full bg-black/30 px-4 py-1">
      <div className="flex items-center gap-2 ml-auto">
        <div
          className={`w-3 h-3 rounded-full ${
            status ? "bg-green-500" : "bg-red-500"
          }`}
        ></div>
        <h1 className="text-sm tracking-wide font-semibold">
          {status ? "Running" : "Stopped"}
        </h1>
      </div>
    </div>
  );
}
