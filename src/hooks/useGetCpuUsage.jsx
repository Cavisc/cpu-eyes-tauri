import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function useGetCpuUsage() {
  const [cpuUsage, setCpuUsage] = useState("...");

  async function getCpuUsage() {
    const cpu_usage = await invoke("get_cpu_usage");
    setCpuUsage(cpu_usage);
  }

  useEffect(() => {
    getCpuUsage();

    const interval = setInterval(() => {
      getCpuUsage();
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return cpuUsage;
}
