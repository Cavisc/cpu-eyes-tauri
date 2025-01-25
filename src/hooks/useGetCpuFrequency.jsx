import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function useGetCpuFrequency() {
  const [cpuFrequency, setCpuFrequency] = useState("...");

  async function getCpuFrequency() {
    const cpu_frequency = await invoke("get_cpu_frequency");
    setCpuFrequency(cpu_frequency);
  }

  useEffect(() => {
    getCpuFrequency();

    const interval = setInterval(() => {
      getCpuFrequency();
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  return cpuFrequency;
}
