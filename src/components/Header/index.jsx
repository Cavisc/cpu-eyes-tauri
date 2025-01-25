import { PiCpuBold } from "react-icons/pi";

import useGetProcessorName from "../../hooks/useGetProcessorName";

export default function Header() {
  const processorName = useGetProcessorName();
  return (
    <header data-tauri-drag-region className="flex justify-center items-center gap-2 select-none">
      <PiCpuBold data-tauri-drag-region className="text-lg select-none" />
      <span data-tauri-drag-region className="text-sm text-center select-none">{processorName}</span>
    </header>
  );
}
