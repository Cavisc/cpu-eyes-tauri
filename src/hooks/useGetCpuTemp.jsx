import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function useGetCpuTemp() {
  const [cpuTemp, setCpuTemp] = useState("...");

  async function getCpuTemp() {
    const cpu_temp = await invoke("get_cpu_temp");
    setCpuTemp(cpu_temp);
  }

  useEffect(() => {
    getCpuTemp();

    const interval = setInterval(() => {
      getCpuTemp();
    }, 7000);

    return () => clearInterval(interval);
  }, []);

  return cpuTemp;
}
