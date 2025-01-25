import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function useGetMemoryUsage() {
  const [memoryUsage, setMemoryUsage] = useState("...");

  async function getMemoryUsage() {
    const memory_usage = await invoke("get_memory_usage");
    setMemoryUsage(memory_usage);
  }

  useEffect(() => {
    getMemoryUsage();

    const interval = setInterval(() => {
      getMemoryUsage();
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  return memoryUsage;
}
