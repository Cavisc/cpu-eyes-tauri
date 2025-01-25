import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function useGetProcessorName() {
  const [processorName, setProcessorName] = useState("...");

  async function getProcessorName() {
    const processor_name = await invoke("get_processor_name");
    setProcessorName(processor_name);
  }

  useEffect(() => {
    getProcessorName();
  }, [getProcessorName]);

  return processorName;
}
